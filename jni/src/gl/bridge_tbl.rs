use crate::gl::gl_bridge::{gl_get_current, gl_init, gl_init_context, gl_make_current, gl_setup_window, gl_swap_buffers, gl_swap_interval};
use crate::GLRenderWindow;
use std::io::Error;
use std::sync::OnceLock;

pub type BrInit = unsafe fn() -> Result<(), Error>;
pub type BrInitContext = unsafe fn(share: Option<&GLRenderWindow>) -> Result<GLRenderWindow, Box<dyn std::error::Error>>;
pub type BrMakeCurrent = unsafe fn(bundle: Option<&mut GLRenderWindow>);
pub type BrGetCurrent = fn() -> *mut GLRenderWindow;
pub type BrSwapBuffers = unsafe fn();
pub type BrSetupWindow = unsafe fn();
pub type BrSwapInterval = unsafe fn(i32);

pub static BR_INIT: OnceLock<Option<BrInit>> = OnceLock::new();
pub static BR_INIT_CONTEXT: OnceLock<Option<BrInitContext>> = OnceLock::new();
pub static BR_MAKE_CURRENT: OnceLock<Option<BrMakeCurrent>> = OnceLock::new();
pub static BR_GET_CURRENT: OnceLock<Option<BrGetCurrent>> = OnceLock::new();
pub static BR_SWAP_BUFFERS: OnceLock<Option<BrSwapBuffers>> = OnceLock::new();
pub static BR_SETUP_WINDOW: OnceLock<Option<BrSetupWindow>> = OnceLock::new();
pub static BR_SWAP_INTERVAL: OnceLock<Option<BrSwapInterval>> = OnceLock::new();

pub fn set_gl_bridge_tbl() {
    BR_INIT.set(Some(gl_init));
    BR_INIT_CONTEXT.set(Some(gl_init_context));
    BR_MAKE_CURRENT.set(Some(gl_make_current));
    BR_GET_CURRENT.set(Some(gl_get_current));
    BR_SWAP_BUFFERS.set(Some(gl_swap_buffers));
    BR_SETUP_WINDOW.set(Some(gl_setup_window));
    BR_SWAP_INTERVAL.set(Some(gl_swap_interval));
}

pub unsafe fn br_init() -> Result<(), Error> {
    (BR_INIT.get().unwrap().unwrap())()
}

pub unsafe fn br_init_context(share: Option<&GLRenderWindow>) -> Result<GLRenderWindow, Box<dyn std::error::Error>> {
    (BR_INIT_CONTEXT.get().unwrap().unwrap())(share)
}

pub unsafe fn br_make_current(bundle: Option<&mut GLRenderWindow>) {
    (BR_MAKE_CURRENT.get().unwrap().unwrap())(bundle)
}

pub unsafe fn br_get_current() -> *const GLRenderWindow {
    (BR_GET_CURRENT.get().unwrap().unwrap())()
}

pub unsafe fn br_swap_buffers() {
    (BR_SWAP_BUFFERS.get().unwrap().unwrap())()
}

pub unsafe fn br_setup_window() {
    (BR_SETUP_WINDOW.get().unwrap().unwrap())()
}

pub unsafe fn br_swap_interval(interval: i32) {
    (BR_SWAP_INTERVAL.get().unwrap().unwrap())(interval)
}
