#![feature(once_cell_get_mut)]

mod environ;
pub mod gl;
mod jre_launcher;
pub mod util;

use crate::gl::egl_loader::EGLFunctions;
use egli::egl::{EGLConfig, EGLContext, EGLSurface, EGLint};
use jni::objects::JString;
use jni::sys::{JNI_FALSE, JNI_TRUE, jboolean};
use jni::{JNIEnv, objects::JClass};
use libloading::Library;
use macros::jni;
use ndk_sys::ANativeWindow;
use std::io::{Error, ErrorKind};
use std::sync::OnceLock;
use util::JNIEnvExt;

// okay, so, listen:
// this code is highly unsafe, and it is a direct port of the C code
// it could be a lot better, but, essentially, it's an experimental version
// so don't expect anything from, it's made to work, not to work well
static EGL_FUNCTIONS: OnceLock<EGLFunctions> = OnceLock::new();

//     char       state;
//     struct ANativeWindow *nativeSurface;
//     struct ANativeWindow *newNativeSurface;
//     EGLConfig  config;
//     EGLint     format;
//     EGLContext context;
//     EGLSurface surface;
#[derive(Debug)]
#[repr(C)]
pub struct GLRenderWindow {
    pub state: i8,
    pub native_surface: Option<*mut ANativeWindow>,
    pub new_native_surface: Option<*mut ANativeWindow>,
    pub config: EGLConfig,
    pub format: EGLint,
    pub context: EGLContext,
    pub surface: EGLSurface,
}

impl GLRenderWindow {
    pub fn new(
        state: i8,
        native_surface: Option<*mut ANativeWindow>,
        new_native_surface: Option<*mut ANativeWindow>,
        config: EGLConfig,
        format: EGLint,
        context: EGLContext,
        surface: EGLSurface,
    ) -> Self {
        Self {
            state,
            native_surface,
            new_native_surface,
            config,
            format,
            context,
            surface,
        }
    }
}

#[repr(C)]
struct BasicRenderWindow {
    pub state: i8,
    pub native_surface: Option<*mut ANativeWindow>,
    pub new_native_surface: Option<*mut ANativeWindow>,
}

#[jni("me.andreasmelone.ponevlauncher.jaba.JREUtils.dlopen")]
pub fn dlopen(mut env: JNIEnv, _class: JClass, path: JString) -> jboolean {
    let path_utf = env.string(&path);

    unsafe {
        match Library::new(&path_utf) {
            Ok(_) => {
                println!("Library loaded: {}", path_utf);
                JNI_TRUE
            }
            Err(e) => {
                eprintln!("Failed to load library: {}. Error: {}", path_utf, e);
                JNI_FALSE
            }
        }
    }
}

#[jni("me.andreasmelone.ponevlauncher.PonevJNI.initLogging")]
pub fn init_logging(env: JNIEnv, _class: JClass) {
    // idk rad will do this hopefully
}

pub fn get_egl_functions() -> Option<*const EGLFunctions> {
    if let Some(functions) = EGL_FUNCTIONS.get() {
        Some(functions)
    } else {
        None
    }
}

pub unsafe fn init_egl() -> Result<(), Error> {
    if EGL_FUNCTIONS.get().is_some() {
        return Ok(());
    }

    let functions = unsafe { EGLFunctions::dlsym_egl() }.unwrap();
    EGL_FUNCTIONS
        .set(functions)
        .map_err(|_| Error::new(ErrorKind::Other, "EGL already initialized!"))
}
