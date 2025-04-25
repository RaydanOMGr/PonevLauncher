use crate::BasicRenderWindow;
use jni::sys::JavaVM;
use ndk_sys::{ANativeWindow};

static mut PONEV_ENVIRON: *mut Environ = std::ptr::null_mut();

#[derive(Default)]
pub struct Environ {
    pub pojav_window: Option<*mut ANativeWindow>,
    pub main_window_bundle: Option<*mut BasicRenderWindow>,
    pub force_vsync: bool,
    pub runtime_java_vm_ptr: Option<*mut JavaVM>,
    pub saved_width: i32,
    pub saved_height: i32,
}

// the function is not unsafe because I don't feel like it
pub fn get_environ() -> &'static mut Environ {
    unsafe {
        if PONEV_ENVIRON.is_null() {
            let b = Box::new(Environ::default());
            PONEV_ENVIRON = Box::into_raw(b);
        }
        &mut *PONEV_ENVIRON
    }
}