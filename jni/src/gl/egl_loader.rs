use egli::egl::{
    EGLBoolean, EGLConfig, EGLContext, EGLDisplay, EGLNativeDisplayType, EGLSurface, EGLenum,
    EGLint,
};
use libloading::{Library, Symbol};
use ndk_sys::ANativeWindow;
use std::ffi::{CString, c_char, c_void};

pub static EGL_WINDOW_BIT: EGLint = 4;
pub static EGL_PBUFFER_BIT: EGLint = 1;
pub static EGL_OPENGL_ES2_BIT: EGLint = 4;

type EglMakeCurrent =
    unsafe extern "C" fn(EGLDisplay, EGLSurface, EGLSurface, EGLContext) -> EGLBoolean;
type EglDestroyContext = unsafe extern "C" fn(EGLDisplay, EGLContext) -> EGLBoolean;
type EglDestroySurface = unsafe extern "C" fn(EGLDisplay, EGLSurface) -> EGLBoolean;
type EglTerminate = unsafe extern "C" fn(EGLDisplay) -> EGLBoolean;
type EglReleaseThread = unsafe extern "C" fn() -> EGLBoolean;
type EglGetCurrentContext = unsafe extern "C" fn() -> EGLContext;
type EglGetDisplay = unsafe extern "C" fn(EGLNativeDisplayType) -> EGLDisplay;
type EglInitialize = unsafe extern "C" fn(EGLDisplay, *mut EGLint, *mut EGLint) -> EGLBoolean;
type EglChooseConfig = unsafe extern "C" fn(
    EGLDisplay,
    *const EGLint,
    *mut EGLConfig,
    EGLint,
    *mut EGLint,
) -> EGLBoolean;
type EglGetConfigAttrib =
    unsafe extern "C" fn(EGLDisplay, EGLConfig, EGLint, *mut EGLint) -> EGLBoolean;
type EglBindApi = unsafe extern "C" fn(EGLenum) -> EGLBoolean;
type EglCreatePbufferSurface =
    unsafe extern "C" fn(EGLDisplay, EGLConfig, *const EGLint) -> EGLSurface;
type EglCreateWindowSurface =
    unsafe extern "C" fn(EGLDisplay, EGLConfig, *const ANativeWindow, *const EGLint) -> EGLSurface;
type EglSwapBuffers = unsafe extern "C" fn(EGLDisplay, EGLSurface) -> EGLBoolean;
type EglGetError = unsafe extern "C" fn() -> EGLint;
type EglCreateContext =
    unsafe extern "C" fn(EGLDisplay, EGLConfig, EGLContext, *const EGLint) -> EGLContext;
type EglSwapInterval = unsafe extern "C" fn(EGLDisplay, EGLint) -> EGLBoolean;
type EglGetCurrentSurface = unsafe extern "C" fn(EGLint) -> EGLSurface;
type EglQuerySurface =
    unsafe extern "C" fn(EGLDisplay, EGLSurface, EGLint, *mut EGLint) -> EGLBoolean;

pub struct EGLFunctions {
    pub egl_make_current: EglMakeCurrent,
    pub egl_destroy_context: EglDestroyContext,
    pub egl_destroy_surface: EglDestroySurface,
    pub egl_terminate: EglTerminate,
    pub egl_release_thread: EglReleaseThread,
    pub egl_get_current_context: EglGetCurrentContext,
    pub egl_get_display: EglGetDisplay,
    pub egl_initialize: EglInitialize,
    pub egl_choose_config: EglChooseConfig,
    pub egl_get_config_attrib: EglGetConfigAttrib,
    pub egl_bind_api: EglBindApi,
    pub egl_create_pbuffer_surface: EglCreatePbufferSurface,
    pub egl_create_window_surface: EglCreateWindowSurface,
    pub egl_swap_buffers: EglSwapBuffers,
    pub egl_get_error: EglGetError,
    pub egl_create_context: EglCreateContext,
    pub egl_swap_interval: EglSwapInterval,
    pub egl_get_current_surface: EglGetCurrentSurface,
    pub eql_query_surface: EglQuerySurface,
}

impl EGLFunctions {
    pub unsafe fn dlsym_egl() -> Result<Self, Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new("libEGL.so")? };
        let get_proc_addr: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_void> =
            unsafe { lib.get(b"eglGetProcAddress")? };

        let load_fn = |name: &str| -> *const c_void {
            let cname = CString::new(name).unwrap();
            unsafe { get_proc_addr(cname.as_ptr()) }
        };

        macro_rules! load {
            ($name:expr, $ty:ty) => {
                std::mem::transmute::<*const c_void, $ty>(load_fn($name))
            };
        }

        unsafe {
            Ok(Self {
                egl_make_current: load!("eglMakeCurrent", EglMakeCurrent),
                egl_destroy_context: load!("eglDestroyContext", EglDestroyContext),
                egl_destroy_surface: load!("eglDestroySurface", EglDestroySurface),
                egl_terminate: load!("eglTerminate", EglTerminate),
                egl_release_thread: load!("eglReleaseThread", EglReleaseThread),
                egl_get_current_context: load!("eglGetCurrentContext", EglGetCurrentContext),
                egl_get_display: load!("eglGetDisplay", EglGetDisplay),
                egl_initialize: load!("eglInitialize", EglInitialize),
                egl_choose_config: load!("eglChooseConfig", EglChooseConfig),
                egl_get_config_attrib: load!("eglGetConfigAttrib", EglGetConfigAttrib),
                egl_bind_api: load!("eglBindAPI", EglBindApi),
                egl_create_pbuffer_surface: load!("eglCreatePbufferSurface", EglCreatePbufferSurface),
                egl_create_window_surface: load!("eglCreateWindowSurface", EglCreateWindowSurface),
                egl_swap_buffers: load!("eglSwapBuffers", EglSwapBuffers),
                egl_get_error: load!("eglGetError", EglGetError),
                egl_create_context: load!("eglCreateContext", EglCreateContext),
                egl_swap_interval: load!("eglSwapInterval", EglSwapInterval),
                egl_get_current_surface: load!("eglGetCurrentSurface", EglGetCurrentSurface),
                eql_query_surface: load!("eglQuerySurface", EglQuerySurface),
            })
        }
    }

    // These methods are marked as unsafe, as they call C functions,
    // which operate outside Rusts safety guarantees
    // C code manages memory manually and does not adhere to Rust's ownership and borrowing rules,
    // so the compiler cannot ensure that these interactions are memory-safe
    pub unsafe fn make_current(
        &self,
        display: EGLDisplay,
        draw: EGLSurface,
        read: EGLSurface,
        context: EGLContext,
    ) -> EGLBoolean {
        unsafe { (self.egl_make_current)(display, draw, read, context) }
    }

    pub unsafe fn destroy_context(&self, display: EGLDisplay, context: EGLContext) -> EGLBoolean {
        unsafe { (self.egl_destroy_context)(display, context) }
    }

    pub unsafe fn destroy_surface(&self, display: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
        unsafe { (self.egl_destroy_surface)(display, surface) }
    }

    pub unsafe fn terminate(&self, display: EGLDisplay) -> EGLBoolean {
        unsafe { (self.egl_terminate)(display) }
    }

    pub unsafe fn release_thread(&self) -> EGLBoolean {
        unsafe { (self.egl_release_thread)() }
    }

    pub unsafe fn get_current_context(&self) -> EGLContext {
        unsafe { (self.egl_get_current_context)() }
    }

    pub unsafe fn get_display(&self, native_display: EGLNativeDisplayType) -> EGLDisplay {
        unsafe { (self.egl_get_display)(native_display) }
    }

    pub unsafe fn initialize(
        &self,
        display: EGLDisplay,
        major: *mut EGLint,
        minor: *mut EGLint,
    ) -> EGLBoolean {
        unsafe { (self.egl_initialize)(display, major, minor) }
    }

    pub unsafe fn choose_config(
        &self,
        display: EGLDisplay,
        attrib_list: *const EGLint,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EGLBoolean {
        unsafe { (self.egl_choose_config)(display, attrib_list, configs, config_size, num_config) }
    }

    pub unsafe fn get_config_attrib(
        &self,
        display: EGLDisplay,
        config: EGLConfig,
        attribute: EGLint,
        value: *mut EGLint,
    ) -> EGLBoolean {
        unsafe { (self.egl_get_config_attrib)(display, config, attribute, value) }
    }

    pub unsafe fn bind_api(&self, api: EGLenum) -> EGLBoolean {
        unsafe { (self.egl_bind_api)(api) }
    }

    pub unsafe fn create_pbuffer_surface(
        &self,
        display: EGLDisplay,
        config: EGLConfig,
        attrib_list: *const EGLint,
    ) -> EGLSurface {
        unsafe { (self.egl_create_pbuffer_surface)(display, config, attrib_list) }
    }

    pub unsafe fn create_window_surface(
        &self,
        display: EGLDisplay,
        config: EGLConfig,
        win: *const ANativeWindow,
        attrib_list: *const EGLint,
    ) -> EGLSurface {
        unsafe { (self.egl_create_window_surface)(display, config, win, attrib_list) }
    }

    pub unsafe fn swap_buffers(&self, display: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
        unsafe { (self.egl_swap_buffers)(display, surface) }
    }

    pub unsafe fn get_error(&self) -> EGLint {
        unsafe { (self.egl_get_error)() }
    }

    pub unsafe fn create_context(
        &self,
        display: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext {
        unsafe { (self.egl_create_context)(display, config, share_context, attrib_list) }
    }

    pub unsafe fn swap_interval(&self, display: EGLDisplay, interval: EGLint) -> EGLBoolean {
        unsafe { (self.egl_swap_interval)(display, interval) }
    }

    pub unsafe fn get_current_surface(&self, readdraw: EGLint) -> EGLSurface {
        unsafe { (self.egl_get_current_surface)(readdraw) }
    }

    pub unsafe fn query_surface(
        &self,
        display: EGLDisplay,
        surface: EGLSurface,
        attribute: EGLint,
        value: *mut EGLint,
    ) -> EGLBoolean {
        unsafe { (self.eql_query_surface)(display, surface, attribute, value) }
    }
}
