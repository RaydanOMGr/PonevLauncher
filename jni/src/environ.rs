use crate::BasicRenderWindow;
use jni::sys::JavaVM;
use ndk_sys::ANativeWindow;
use std::cell::Cell;
use std::sync::atomic::AtomicPtr;
use std::sync::OnceLock;

static PONEV_ENVIRON: OnceLock<AtomicPtr<Environ>> = OnceLock::new();

#[derive(Default)]
pub struct Environ {
    pub pojav_window: Cell<Option<*mut ANativeWindow>>,
    pub main_window_bundle: Cell<Option<*mut BasicRenderWindow>>,
    pub force_vsync: Cell<bool>,
    pub runtime_java_vm_ptr: Cell<Option<*mut JavaVM>>,
    pub saved_width: Cell<i32>,
    pub saved_height: Cell<i32>,
}

pub fn get_environ() -> *mut Environ {
    unsafe {
        *PONEV_ENVIRON.get_or_init(|| {
            let environ = Box::new(Environ::default());
            AtomicPtr::new(Box::into_raw(environ))
        }).as_ptr()
    }
}