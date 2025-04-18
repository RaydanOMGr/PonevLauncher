pub mod util;

use jni::{objects::{JClass, JString}, sys::jstring, JNIEnv};
use util::JNIEnvExt;

#[no_mangle]
pub extern "C" fn Java_me_andreasmelone_ponevlauncher_JNI_sayHello<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
) -> jstring {
    let name = env.string(&name);
    let output = env.new_string(format!("hello, {name}!")).expect("failed creating string");

    output.into_raw()
}
