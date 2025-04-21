use crate::BasicRenderWindow;
use jni::sys::JavaVM;
use ndk_sys::ANativeWindow;
use std::cell::Cell;
use std::sync::OnceLock;

static mut PONEV_ENVIRON: OnceLock<Environ> = OnceLock::new();

#[derive(Default)]
pub struct Environ {
    pub pojav_window: Cell<Option<*mut ANativeWindow>>,
    pub main_window_bundle: Cell<Option<*mut BasicRenderWindow>>,
    pub force_vsync: Cell<bool>,
    pub runtime_java_vm_ptr: Cell<Option<*mut JavaVM>>,
    pub saved_width: Cell<i32>,
    pub saved_height: Cell<i32>,
}

pub fn get_environ() -> &'static mut Environ {
    unsafe { PONEV_ENVIRON.get_mut_or_init(|| Environ::default()) }
}
