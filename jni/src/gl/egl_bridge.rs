use crate::environ::get_environ;
use crate::gl::bridge_tbl::{br_get_current, br_init, br_init_context, br_make_current, br_setup_window, br_swap_buffers, br_swap_interval, set_gl_bridge_tbl};
use crate::util::get_attached_env;
use crate::{get_egl_functions, GLRenderWindow};
use egli::egl::{EGL_NO_CONTEXT, EGL_NO_DISPLAY, EGL_NO_SURFACE};
use jni::objects::{JClass, JObject};
use jni::JNIEnv;
use macros::jni;
use ndk_sys::{ANativeWindow_acquire, ANativeWindow_fromSurface, ANativeWindow_getHeight, ANativeWindow_getWidth, ANativeWindow_release, ANativeWindow_setBuffersGeometry};
use std::env;

const AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM: i32 = 2;

unsafe fn ponev_terminate() {
    let egl = get_egl_functions().unwrap();
    println!("EGLBridge: Terminating");

    (*egl).make_current(EGL_NO_DISPLAY, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
    (*egl).destroy_surface(EGL_NO_DISPLAY, EGL_NO_SURFACE);
    (*egl).destroy_context(EGL_NO_DISPLAY, EGL_NO_CONTEXT);
    (*egl).terminate(EGL_NO_DISPLAY);
    (*egl).release_thread();
}

#[jni("me.andreasmelone.ponevlauncher.jaba.JREUtils.setupBridgeWindow")]
pub fn setup_bridge_window(mut env: JNIEnv, _class: JClass, surface: JObject) {
    unsafe {
        (*get_environ()).pojav_window.set(Some(ANativeWindow_fromSurface(env.get_raw(), *surface)));
    }
}

#[jni("me.andreasmelone.ponevlauncher.jaba.JREUtils.releaseBridgeWindow")]
pub unsafe fn release_bridge_window(env: JNIEnv, _class: JClass) {
    unsafe {
        let environ = get_environ();
        ANativeWindow_release((*environ).pojav_window.get().unwrap());
        (*environ).pojav_window.set(None);
    }
}

fn ponev_get_current_context() -> unsafe fn() -> *const GLRenderWindow {
    br_get_current
}

unsafe fn ponev_init_opengl() {
    let force_vsync = env::var("FORCE_VSYNC").unwrap();
    (*get_environ()).force_vsync.set(force_vsync == "true");

    set_gl_bridge_tbl();

    if br_init().is_ok() {
        br_setup_window();
    }
}

unsafe fn ponev_swap_buffers() {
    br_swap_buffers();
}

unsafe fn ponev_make_current(window: *mut GLRenderWindow) {
    br_make_current(
        if window.is_null() { None }
        else { Some(&mut *window) }
    );
}

unsafe fn ponev_create_context(context_src: *mut GLRenderWindow) -> GLRenderWindow {
    br_init_context(
        if context_src.is_null() { None }
        else { Some(&*context_src) }
    ).unwrap()
}

unsafe fn ponev_init() -> bool {
    let environ = get_environ();
    let glfw_thread_env = get_attached_env((*environ).runtime_java_vm_ptr.get().unwrap());
    if let None = glfw_thread_env {
        return false;
    }

    let window = (*environ).pojav_window.get().unwrap();
    ANativeWindow_acquire(window);
    (*environ).saved_width.set(ANativeWindow_getWidth(window));
    (*environ).saved_height.set(ANativeWindow_getHeight(window));

    ANativeWindow_setBuffersGeometry(window, (*environ).saved_width.get(), (*environ).saved_height.get(), AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM);
    update_monitor_size((*environ).saved_width.get(), (*environ).saved_height.get());
    ponev_init_opengl();

    true
}

fn ponev_set_window_hint(hint: i32, value: i32) {
    // empty, required for ffi purposes
}

unsafe fn ponev_swap_interval(interval: i32) {
    br_swap_interval(interval);
}

unsafe extern "C" {
    fn update_monitor_size(width: i32, height: i32);
}