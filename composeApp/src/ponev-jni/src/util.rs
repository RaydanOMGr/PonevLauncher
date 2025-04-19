use jni::{objects::JString, JNIEnv};

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
