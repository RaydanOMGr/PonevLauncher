pub mod util;

use jni::{
    InitArgsBuilder, JNIEnv, JavaVM,
    objects::{JClass, JObjectArray, JString, JValue},
    sys::jstring,
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

#[jni("me.andreasmelone.ponevlauncher.JNI.spawnJvm")]
pub fn spawn_jvm(
    mut env: JNIEnv,
    _class: JClass,
    jvm_flags: JObjectArray,
    program_args: JObjectArray,
    main_class: JString,
) {
    let jvm_flags = env.string_array(&jvm_flags);
    let program_args = env.string_array(&program_args);

    let main_class_str = env.string(&main_class);

    let mut jvm_args = InitArgsBuilder::new().version(jni::JNIVersion::V8);
    for flag in jvm_flags {
        jvm_args.try_option(flag).expect("invalid jvm flag");
    }

    let jvm_args = jvm_args.build().unwrap();
    let jvm = JavaVM::new(jvm_args).expect("failed creating JVM");

    let mut env = jvm
        .attach_current_thread()
        .expect("failed attaching to JVM");

    let main_class = env
        .find_class(&main_class_str)
        .expect("could not find main class");
    let program_args = env.new_string_array(&program_args);

    env.call_static_method(
        main_class,
        "main",
        "([Ljava/lang/String;)V",
        &[JValue::Object(&program_args)],
    )
    .expect("failed to call main method");
}
