use jni::errors::{Error, JniError};
use jni::objects::{GlobalRef, JClass, JObject};
use jni::sys::{jboolean, jclass, jint, JNI_FALSE, JNI_TRUE};
use jni::{JNIEnv, JavaVM};
use libc::{c_int, getpgrp, kill, pid_t, sigaction, sigemptyset, sighandler_t, SIGABRT, SIGHUP, SIGSEGV, SIG_DFL, SIG_IGN};
use libloading::{Library, Symbol};
use ndk_sys::NSIG;
use nix::sys::signal::Signal;
use signal_hook::iterator::Signals;
use std::io::{pipe, PipeReader, PipeWriter, Read, Write};
use std::sync::{Arc, Mutex};
use std::{ffi::CString, io, os::raw::c_char, process, ptr, thread};
use macros::jni;

type JLILaunch = extern "C" fn(
    argc: i32, argv: *const *const c_char,            /* main argc, argc */
    jargc: i32, jargv: *const *const c_char,          /* java args */
    appclassc: i32, appclassv: *const *const c_char,  /* app classpath */
    fullversion: *const c_char,                       /* full version defined */
    dotversion: *const c_char,                        /* dot version defined */
    pname: *const c_char,                             /* program name */
    lname: *const c_char,                             /* launcher name */
    javaargs: jboolean,                               /* JAVA_ARGS */
    cpwildcard: jboolean,                             /* classpath wildcard */
    javaw: jboolean,                                  /* windows-only javaw */
    ergo: jint                                        /* ergonomics class policy */
) -> jint;

const FULL_VERSION: &str = "1.8.0-internal";
const DOT_VERSION: &str = "1.8";

static CONST_PROGNAME: &str = "java";
static CONST_LAUNCHER: &str = "openjdk";
static CONST_JARGS: Option<Vec<String>> = None;
static CONST_APPCLASSPATH: Vec<String> = Vec::new();
static CONST_JAVAW: jboolean = JNI_FALSE;
static CONST_CPWILDCARD: jboolean = JNI_TRUE;
static CONST_ERGO_CLASS: jint = 0; // DEFAULT_POLICY

static EXIT_TRAP_VM: Mutex<Option<JavaVM>> = Mutex::new(None);
static EXIT_TRAP_CTX: Mutex<Option<GlobalRef>> = Mutex::new(None);
static EXIT_TRAP_CLASS: Mutex<Option<JClass>> = Mutex::new(None);

struct AbortWaiterData {
    tracked_sigset: Vec<i32>,
    reader: PipeReader,
    writer: PipeWriter
}

impl AbortWaiterData {
    fn new() -> io::Result<Self> {
        let (read_fd, write_fd) = pipe()?;
        Ok(AbortWaiterData {
            tracked_sigset: vec![SIGABRT],
            reader: read_fd,
            writer: write_fd,
        })
    }
}

fn abort_waiter_thread(abort_waiter_data: Arc<Mutex<AbortWaiterData>>) {
    let mut buffer = [0; size_of::<i32>()];

    unsafe {
        libc::pthread_sigmask(libc::SIG_BLOCK, &abort_waiter_data.lock().unwrap().tracked_sigset as *const _ as *const _, std::ptr::null_mut());
    }

    loop {
        let mut data = abort_waiter_data.lock().unwrap();
        let read_len = data.reader.read(&mut buffer).unwrap();
        if read_len as i32 == -1 {
            eprintln!("Failed to read from pipe");
            break;
        }

        let signal = i32::from_ne_bytes(buffer);
        nominal_exit(signal, true);
    }
}

fn abort_waiter_handler(signal: i32, abort_waiter_data: Arc<Mutex<AbortWaiterData>>) {
    let mut buffer = [0; size_of::<i32>()];
    buffer.copy_from_slice(&signal.to_ne_bytes());

    let mut data = abort_waiter_data.lock().unwrap();
    let write_len = data.writer.write(&mut buffer).unwrap();
    if write_len as i32 == -1 {
        eprintln!("Failed to write to pipe");
    }

    loop {}
}

fn abort_waiter_setup() -> io::Result<()> {
    let abort_waiter_data = Arc::new(Mutex::new(AbortWaiterData::new()?));

    let signals = Arc::new(Mutex::new(Signals::new(&[SIGABRT])?));

    let abort_waiter_data_clone = abort_waiter_data.clone();
    thread::spawn(move || {
        abort_waiter_thread(abort_waiter_data_clone);
    });

    let signals_clone = signals.clone();
    thread::spawn(move || {
        let mut signals = signals_clone.lock().unwrap();
        for signal in signals.forever() {
            abort_waiter_handler(signal, abort_waiter_data.clone());
        }
    });

    Ok(())
}

pub unsafe fn launch_jvm(margc: i32, margv: *const *const c_char) -> i32 {
    unsafe {
        let libjli = match Library::new("libjli.so") {
            Ok(lib) => lib,
            Err(e) => {
                eprintln!("JLI lib = NULL: {}", e);
                return -1;
            }
        };
        println!("Found JLI lib");


        let mut clean_sa: sigaction = std::mem::zeroed();
        sigemptyset(&mut clean_sa.sa_mask);
        clean_sa.sa_flags = 0;

        for sigid in SIGHUP as u32..NSIG {
            clean_sa.sa_sigaction = if sigid == SIGSEGV as u32 {
                SIG_IGN as sighandler_t
            } else {
                SIG_DFL as sighandler_t
            };
            sigaction(sigid as c_int, &clean_sa, ptr::null_mut());
        }

        abort_waiter_setup();

        let launch: Symbol<JLILaunch> = match libjli.get(b"JLILaunch") {
            Ok(sym) => sym,
            Err(_) => {
                eprintln!("JLILaunch = NULL");
                return -1;
            }
        };

        println!("Calling JLILaunch");

        let full_version_c = CString::new(FULL_VERSION).unwrap();
        let dot_version_c = CString::new(DOT_VERSION).unwrap();
        let progname = *margv;
        let launcher = *margv;

        launch(
            margc,
            margv,
            0,
            ptr::null(),
            0,
            ptr::null(),
            full_version_c.as_ptr(),
            dot_version_c.as_ptr(),
            progname,
            launcher,
            if !CONST_JARGS.is_some() { 1 } else { 0 },
            CONST_CPWILDCARD,
            CONST_JAVAW,
            CONST_ERGO_CLASS,
        )
    }
}

fn nominal_exit(code: i32, is_signal: bool) {
    let exit_trap_jvm_guard = EXIT_TRAP_VM.lock().unwrap();
    let exit_trap_jvm = exit_trap_jvm_guard.as_ref().unwrap();

    let mut env: JNIEnv;
    match exit_trap_jvm.get_env() {
        Ok(result) => {
            env = result;
        }
        Err(err) => {
            match err {
                Error::JniCall(ref source) => {
                    match source {
                        JniError::ThreadDetached => {
                            let thread_attach_code = exit_trap_jvm.attach_current_thread();
                            if let Err(_) = thread_attach_code {
                                unsafe {
                                    let child_pid = getpgrp() as pid_t;
                                    kill(child_pid, Signal::SIGTERM as c_int);
                                }
                                process::exit(1);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    process::exit(1);
                }
            }
            return;
        }
    }

    if code != 0 {
        let exit_class_guard = EXIT_TRAP_CLASS.lock().unwrap();
        let exit_class = exit_class_guard.as_ref().unwrap();
        let ctx_guard = EXIT_TRAP_CTX.lock().unwrap();
        let ctx = ctx_guard.as_ref().unwrap().as_obj();

        // Exit code 0 is pretty established as "eh it's fine"
        // so only open the GUI if the code is != 0
        env.call_static_method(
            exit_class,
            "showExitMessage",
            "(Landroid/content/Context;IZ)V",
            &[ctx.into(), code.into(), is_signal.into()]
        ).unwrap().v().unwrap();
    }
    // Delete the reference, not gonna need 'em later anyway
    *EXIT_TRAP_CTX.lock().unwrap() = None;
    *EXIT_TRAP_CLASS.lock().unwrap() = None;

    // A hat trick, if you will
    // Call the Android System.exit() to perform Android's shutdown hooks and do a
    // fully clean exit.
    // After doing this, either of these will happen:
    // 1. Runtime calls exit() for real and it will be handled by ByteHook's recurse handler
    // and redirected back to the OS
    // 2. Zygote sends SIGTERM (no handling necessary, the process perishes)
    // 3. A different thread calls exit() and the hook will go through the exit_tripped path
    let system_class = env.find_class("java/lang/System").unwrap();
    env.call_static_method(system_class, "exit", "(I)V", &[0.into()]).unwrap().v().unwrap();
    // System.exit() should not ever return, but the compiler doesn't know about that
    // so put a while loop here
    loop {}
}

#[jni("me.andreasmelone.ponevlauncher.jaba.setupExitMethod")]
fn setup_exit_method(mut env: JNIEnv, clazz: jclass, context: JObject) {
    {
        let mut lock = EXIT_TRAP_CTX.lock().unwrap();
        let ctx = env.new_global_ref(context).unwrap();
        *lock = Some(ctx);
    }
    {
        let mut lock = EXIT_TRAP_VM.lock().unwrap();
        *lock = Some(env.get_java_vm().unwrap());
    }
    unsafe {
        let mut lock = EXIT_TRAP_CLASS.lock().unwrap();
        let class = env.find_class("net/kdt/pojavlaunch/ExitActivity").unwrap();
        let global_ref = env.new_global_ref(class).unwrap();
        let jclass_ref = unsafe { JClass::from_raw(global_ref.as_raw() as jclass) }; // let's pretend it's safe because it is supposed to be
        *lock = Some(jclass_ref);
    }
}