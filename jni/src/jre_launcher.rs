use libloading::{Library, Symbol};
use libc::{sigaction, sighandler_t, sigemptyset, SIG_DFL, SIG_IGN, SIGHUP, SIGSEGV, c_int, SIGABRT};
use std::{ffi::CString, os::raw::c_char, ptr};
use ndk_sys::NSIG;

type JLILaunch = unsafe extern "C" fn(
    argc: i32,
    argv: *const *const c_char,
    jargc: i32,
    jargv: *const *const c_char,
    appcp: i32,
    appclasspath: *const *const c_char,
    fullversion: *const c_char,
    dotversion: *const c_char,
    progname: *const c_char,
    launcher: *const c_char,
    jargs: u8,
    cpwildcard: u8,
    javaw: u8,
    ergo: u8,
) -> i32;


static const_jargs: *const *const c_char = ptr::null();
static const_appclasspath: *const *const c_char = ptr::null();
static const_cpwildcard: u8 = 0;
static const_javaw: u8 = 0;
static const_ergo_class: u8 = 0;
static FULL_VERSION: &str = "1.8.0";
static DOT_VERSION: &str = "1.8";

pub unsafe fn launch_jvm(margc: i32, margv: *const *const c_char) -> i32 {
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
        if !const_jargs.is_null() { 1 } else { 0 },
        const_cpwildcard,
        const_javaw,
        const_ergo_class,
    )
}