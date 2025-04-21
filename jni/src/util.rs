use std::ptr;
use jni::{objects::JString, JNIEnv};
use jni::sys::{JavaVM, JNI_EDETACHED, JNI_OK, JNI_VERSION_1_4};

pub trait JNIEnvExt<'local> {
    fn string<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JString<'other_local>,
    ) -> String;
}

impl<'local> JNIEnvExt<'local> for JNIEnv<'local> {
    fn string<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JString<'other_local>,
    ) -> String {
        self.get_string(obj).expect("failed to get string").into()
    }
}

pub fn get_attached_env(jvm: *mut JavaVM) -> Option<*mut JNIEnv<'static>> {
    let jvm_env = ptr::null_mut();
    let mut env_result = unsafe { ((*(*jvm)).GetEnv.unwrap())(jvm, jvm_env, JNI_VERSION_1_4) };
    if env_result == JNI_EDETACHED {
        env_result = unsafe { ((*(*jvm)).AttachCurrentThread.unwrap())(jvm, jvm_env, ptr::null_mut()) };
    }
    if env_result != JNI_OK {
        println!("get_attached_env failed: {}", env_result);
        return None;
    }

    Some(jvm_env as *mut JNIEnv)
}