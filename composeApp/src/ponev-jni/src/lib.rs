pub mod util;

use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};
use macros::jni;
use util::JNIEnvExt;

#[jni("me.andreasmelone.ponevlauncher.JNI.sayHello")]
pub fn say_hello(mut env: JNIEnv, _class: JClass, name: JString) -> jstring {
    let name = env.string(&name);
    let output = env
        .new_string(format!("hello, {name}!"))
        .expect("failed creating string");

    output.into_raw()
}
