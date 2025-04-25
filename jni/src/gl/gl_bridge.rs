use crate::gl::egl_loader::{EGL_OPENGL_ES2_BIT, EGL_PBUFFER_BIT, EGL_WINDOW_BIT};
use crate::{get_egl_functions, init_egl, BasicRenderWindow, GLRenderWindow};
use egli::egl::{
    EGLBoolean, EGLConfig, EGLDisplay, EGLint,
    EGL_ALPHA_SIZE, EGL_BAD_SURFACE, EGL_BLUE_SIZE, EGL_CONTEXT_CLIENT_VERSION, EGL_DEFAULT_DISPLAY,
    EGL_DEPTH_SIZE, EGL_GREEN_SIZE, EGL_HEIGHT, EGL_NATIVE_VISUAL_ID, EGL_NONE, EGL_NO_CONTEXT,
    EGL_NO_DISPLAY, EGL_NO_SURFACE, EGL_OPENGL_API, EGL_OPENGL_ES_API, EGL_RED_SIZE, EGL_RENDERABLE_TYPE,
    EGL_SURFACE_TYPE, EGL_TRUE, EGL_WIDTH,
};
use jni::objects::{JClass, JObject, JValue};
use jni::JNIEnv;
use macros::jni;
use ndk_sys::{ANativeWindow_acquire, ANativeWindow_release, ANativeWindow_setBuffersGeometry};
use std::io::{Error, ErrorKind};
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, Mutex};
use std::{env, ptr};
use crate::environ::get_environ;

static EGL_DISPLAY: Mutex<Option<AtomicPtr<EGLDisplay>>> = Mutex::new(None);
thread_local! {
    static CURRENT_BUNDLE: Arc<Mutex<Option<*mut GLRenderWindow>>> = Arc::new(Mutex::new(None));
}

const STATE_RENDERER_ALIVE: i8 = 0;
const STATE_RENDERER_NEW_WINDOW: i8 = 1;

pub unsafe fn gl_init() -> Result<(), Error> {
    unsafe {
    init_egl()?;
    }
    let egl = get_egl_functions().unwrap();

    let display = unsafe { (*egl).get_display(EGL_DEFAULT_DISPLAY) };
    let mut guard = EGL_DISPLAY.lock().unwrap();
    *guard = Some(AtomicPtr::new(display as *mut EGLDisplay));

    if display == EGL_NO_DISPLAY {
        return Err(Error::new(
            ErrorKind::Other,
            "eglGetDisplay(EGL_DEFAULT_DISPLAY) returned EGL_NO_DISPLAY",
        ));
    }

    unsafe {
        if (*egl).initialize(display, 0 as *mut EGLint, 0 as *mut EGLint) != EGL_TRUE {
            let err = (*egl).get_error();
            return Err(Error::new(
                ErrorKind::Other,
                format!("eglInitialize_p() failed: {err}"),
            ));
        }
    }

    Ok(())
}

pub fn gl_get_current() -> *mut GLRenderWindow {
    CURRENT_BUNDLE.with(|b| b.lock().unwrap().unwrap())
}

pub unsafe extern "C" fn gl4esi_get_display_dimensions(width: &mut i32, height: &mut i32) {
    let egl = get_egl_functions().unwrap();

    unsafe {
        let display_guard = EGL_DISPLAY.lock().unwrap();
        let display = *display_guard.as_ref().unwrap().as_ptr();
        let bundle_guard = CURRENT_BUNDLE.with(|b| b.lock().unwrap().clone());
        if bundle_guard.is_none() {
            *width = 0;
            *height = 0;
            return;
        }
    
        let surface = (*bundle_guard.unwrap()).surface;
    
        let result_width = (*egl).query_surface(*display, surface, EGL_WIDTH, width);
        let result_height = (*egl).query_surface(*display, surface, EGL_HEIGHT, height);
        if result_width != EGL_TRUE || result_height != EGL_TRUE {
            *width = 0;
            *height = 0;
        }
    }
}

pub unsafe fn gl_init_context(
    share: Option<&GLRenderWindow>,
) -> Result<GLRenderWindow, Box<dyn std::error::Error>> {
    let mut bundle: Box<GLRenderWindow> = Box::new(unsafe { std::mem::zeroed() });

    let egl = get_egl_functions().unwrap();
    let display_guard = EGL_DISPLAY.lock().unwrap();
    let display = unsafe { *display_guard.as_ref().unwrap().as_ptr() };

    let egl_attributes = [
        EGL_BLUE_SIZE,
        8,
        EGL_GREEN_SIZE,
        8,
        EGL_RED_SIZE,
        8,
        EGL_ALPHA_SIZE,
        8,
        EGL_DEPTH_SIZE,
        24,
        EGL_SURFACE_TYPE,
        EGL_WINDOW_BIT | EGL_PBUFFER_BIT,
        EGL_RENDERABLE_TYPE,
        EGL_OPENGL_ES2_BIT,
        EGL_NONE,
    ]
    .as_ptr();
    let num_configs = 0 as *mut EGLint;

    unsafe {
        if (*egl).choose_config(*display, egl_attributes, ptr::null_mut(), 0, num_configs) != EGL_TRUE { let err = (*egl).get_error();
            return Err(
                Error::new(ErrorKind::Other, format!("eglChooseConfig() failed: {err}")).into(),
            );
        }
    }
    if num_configs == 0 as *mut EGLint {
        return Err(Error::new(
            ErrorKind::Other,
            "eglChooseConfig() found no matching config",
        )
        .into());
    }

    unsafe {
        (*egl).choose_config(
            *display,
            egl_attributes,
            &mut bundle.config as *mut EGLConfig,
            1,
            num_configs,
        );
        (*egl).get_config_attrib(
            *display,
            bundle.config,
            EGL_NATIVE_VISUAL_ID,
            &mut bundle.format as *mut EGLint,
        );
    }

    let bind_result: EGLBoolean;
    if env::var("MOJO_RENDERER")? == "opengles3_desktopgl" {
        println!("EGLBridge: Binding to desktop OpenGL\n");
        bind_result = unsafe { (*egl).bind_api(EGL_OPENGL_API) };
    } else {
        println!("EGLBridge: Binding to OpenGL ES\n");
        bind_result = unsafe { (*egl).bind_api(EGL_OPENGL_ES_API) };
    }
    if bind_result != EGL_TRUE {
        let err = unsafe { (*egl).get_error() };
        println!("EGLBridge: bind failed: {err}\n");
    }

    let mut libgl_es = str::parse(&*env::var("LIBGL_ES")?)?;
    if libgl_es < 0 || libgl_es > i16::MAX as i32 {
        libgl_es = 2;
    }
    let egl_context_attributes = [EGL_CONTEXT_CLIENT_VERSION, libgl_es, EGL_NONE].as_ptr();
    bundle.context = unsafe {(*egl).create_context(
        *display,
        bundle.config,
        if let Some(value) = share {
            (&value).context
        } else {
            EGL_NO_CONTEXT
        },
        egl_context_attributes,
    )};

    if bundle.context == EGL_NO_CONTEXT {
        let err = unsafe { (*egl).get_error() };
        return Err(Error::new(
            ErrorKind::Other,
            format!("eglCreateContext() finished with error: {err}"),
        )
        .into());
    }

    Ok(*bundle)
}

pub unsafe fn gl_swap_surface(bundle: &mut GLRenderWindow) {
    let egl = get_egl_functions().unwrap();
    let display_guard = EGL_DISPLAY.lock().unwrap();
    let display = unsafe { *display_guard.as_ref().unwrap().as_ptr() };

    if let Some(native_surface) = (&bundle).native_surface {
        unsafe {
        ANativeWindow_release(native_surface);
        }
    }
    if !(&bundle).surface.is_null() {
        unsafe { (*egl).destroy_surface(*display, (&bundle).surface) };
    }
    if let Some(new_surface) = (&bundle).new_native_surface {
        println!("Switching to new native surface");
        bundle.native_surface = Some(new_surface);
        bundle.new_native_surface = None;
        unsafe {
            ANativeWindow_acquire(bundle.native_surface.unwrap());
            ANativeWindow_setBuffersGeometry(bundle.native_surface.unwrap(), 0, 0, bundle.format);
        }
        bundle.surface = unsafe {
            (*egl).create_window_surface(
                *display,
                bundle.config,
                bundle.native_surface.unwrap(),
                ptr::null_mut(),
            )
        };
    } else {
        println!("No new native surface, switching to 1x1 pbuffer");
        bundle.native_surface = None;
        let pbuffer_attrs = [EGL_WIDTH, 1, EGL_HEIGHT, 1, EGL_NONE].as_ptr();
        bundle.surface =
            unsafe { (*egl).create_pbuffer_surface(*display, bundle.config, pbuffer_attrs) };
    }
}

pub unsafe fn gl_make_current(bundle: Option<&mut GLRenderWindow>) {
    let mut environ = get_environ();
    let egl = get_egl_functions().unwrap();
    let display_guard = EGL_DISPLAY.lock().unwrap();
    let display = unsafe { *display_guard.as_ref().unwrap().as_ptr() };

    if (&bundle).is_none() {
        unsafe {
            if (*egl).make_current(*display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT)
                == EGL_TRUE
            {
                CURRENT_BUNDLE.with(|bundle| *bundle.lock().unwrap() = None);
            }
        }
        return;
    }

    let mut unwrapped = bundle.unwrap();
    let mut has_set_main_window = false;

    if (*environ).main_window_bundle.is_none() {
        let ptr: *const GLRenderWindow = unwrapped;
        (*environ).main_window_bundle = Some(ptr as *mut BasicRenderWindow);
        println!(
            "Main window bundle is now {:?}",
            (*environ).main_window_bundle
        );
        // main_window_bundle->new_native_surface = environ->pojav_window;
        let main_window_bundle = (*environ).main_window_bundle.take().unwrap();
        unsafe {
            (*main_window_bundle).new_native_surface = Some((*environ).pojav_window.unwrap());
        }

        has_set_main_window = true;
    }
    println!(
        "Making current, surface={:?}, nativeSurface={:?} newNativeSurface={:?}",
        unwrapped.surface, unwrapped.native_surface, unwrapped.new_native_surface
    );
    unsafe {
        if unwrapped.surface.is_null() {
            gl_swap_surface(&mut unwrapped);
        }
        if (*egl).make_current(
            *display,
            unwrapped.surface,
            unwrapped.surface,
            unwrapped.context,
        ) == EGL_TRUE
        {
            CURRENT_BUNDLE.with(|bundle| *bundle.lock().unwrap() = None);
        } else {
            if has_set_main_window {
                let main_window = (*environ).main_window_bundle.unwrap();
                (*main_window).new_native_surface = *ptr::null_mut();
                //gl_render_window_t*
                gl_swap_surface(&mut *(main_window as *mut GLRenderWindow));
                (*environ).main_window_bundle = None;
            }
            let err = (*egl).get_error();
            println!("eglMakeCurrent returned with error: {err}");
        }
    }
}

pub unsafe fn gl_swap_buffers() {
    let egl = get_egl_functions().unwrap();
    let display_guard = EGL_DISPLAY.lock().unwrap();
    let display = unsafe { *display_guard.as_ref().unwrap().as_ptr() };
    &CURRENT_BUNDLE.with(|b| {
        let bundle_guard = (*b.lock().unwrap()).unwrap();

        unsafe {
            if (*bundle_guard).state == STATE_RENDERER_NEW_WINDOW {
                (*egl).make_current(*display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT); //detach everything to destroy the old EGLSurface
                gl_swap_surface(&mut *bundle_guard);
                (*egl).make_current(
                    *display,
                    (*bundle_guard).surface,
                    (*bundle_guard).surface,
                    (*bundle_guard).context,
                );
                (*bundle_guard).state = STATE_RENDERER_ALIVE;
            }
            if !(*bundle_guard).surface.is_null()
                && (*egl).swap_buffers(*display, (*bundle_guard).surface) != EGL_TRUE
                && (*egl).get_error() == EGL_BAD_SURFACE
            {
                (*egl).make_current(*display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
                (*bundle_guard).new_native_surface = None;
                gl_swap_surface(&mut *bundle_guard);
                (*egl).make_current(
                    *display,
                    (*bundle_guard).surface,
                    (*bundle_guard).surface,
                    (*bundle_guard).context,
                );
                println!("The window has died, awaiting window change");
            }
        }
    });
}

pub unsafe fn gl_setup_window() {
    let environ = get_environ();

    if let Some(window_bundle) = (*environ).main_window_bundle {
        println!("Main window bundle is not NULL, changing state");
        unsafe {
            (*window_bundle).state = STATE_RENDERER_NEW_WINDOW;
            (*window_bundle).new_native_surface = Some((*environ).pojav_window.unwrap());
        }
    }
}

pub unsafe fn gl_swap_interval(mut swap_interval: i32) {
    let egl = get_egl_functions().unwrap();
    let display_guard = EGL_DISPLAY.lock().unwrap();
    let display = unsafe { *display_guard.as_ref().unwrap().as_ptr() };
    if get_environ().force_vsync {
        swap_interval = 1;
    }

    unsafe { (*egl).swap_interval(*display, swap_interval); }
}

#[jni("me.andreasmelone.ponevlauncher.game.RendererInit.nativeInitGl4esInternals")]
pub fn native_init_gl4es_internals(mut env: JNIEnv, _class: JClass, function_provider: JObject) {
    println!("GL4ES internals initializing...");
    let _func_provider_class = env.get_object_class(&function_provider).unwrap();

    let get_sym = |n: &str| {
        {
            let strng = env.new_string(n).unwrap();
            let args = &[JValue::Object(&*strng)];
            env.call_method(
                function_provider,
                "getFunctionAddress",
                "(Ljava/lang/CharSequence;)J",
                args,
            )
        }
        .unwrap()
        .j()
        .unwrap()
    };

    let set_getmainfbsize: Option<unsafe extern "C" fn(unsafe extern "C" fn(&mut i32, &mut i32))> =
        unsafe { std::mem::transmute(get_sym("set_getmainfbsize") as *mut ()) };

    if let Some(set_getmainfbsize_fn) = set_getmainfbsize {
        println!("GL4ES internals initialized dimension callback");
        unsafe {
            set_getmainfbsize_fn(gl4esi_get_display_dimensions);
        }
    }
}
