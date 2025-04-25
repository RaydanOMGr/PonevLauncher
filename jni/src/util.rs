use jni::objects::JObjectArray;
use jni::sys::{jclass, JavaVM, JNI_EDETACHED, JNI_OK, JNI_VERSION_1_4};
use jni::{objects::JString, JNIEnv};
use libc::c_char;
use std::ffi::CString;
use std::ptr;

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
        env_result =
            unsafe { ((*(*jvm)).AttachCurrentThread.unwrap())(jvm, jvm_env, ptr::null_mut()) };
    }
    if env_result != JNI_OK {
        println!("get_attached_env failed: {}", env_result);
        return None;
    }

    Some(jvm_env as *mut JNIEnv)
}

pub fn convert_to_string_array(mut env: JNIEnv, string_array: JObjectArray) -> Vec<String> {
    let size = (&env).get_array_length(&string_array).unwrap();
    let mut strings: Vec<String> = Vec::new();

    for i in 0..size as i32 {
        let obj = (&mut env).get_object_array_element(&string_array, i).unwrap();
        let jstr = unsafe { JString::from_raw(obj.as_raw() as jclass) };
        let str = (&mut env).string(&jstr);
        strings.push(str);
    }

    strings
}

pub fn vec_to_c_array(vec: Vec<String>) -> *const *const c_char {
    let cstrings: Vec<CString> = (&vec).into_iter()
        .map(|s| CString::new(s.to_string()).unwrap())
        .collect();

    let mut ptrs: Vec<*const c_char> = Vec::with_capacity(vec.len());
    for cstr in cstrings {
        ptrs.push(cstr.into_raw());
    }

    ptrs.push(ptr::null());
    ptrs.as_ptr()
}