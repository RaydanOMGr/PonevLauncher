use jni::{
    JNIEnv,
    objects::{JObject, JObjectArray, JString},
    sys::jint,
};

pub trait JNIEnvExt<'local> {
    fn string<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JString<'other_local>,
    ) -> String;

    fn string_array<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        array: &'obj_ref JObjectArray<'other_local>,
    ) -> Vec<String>;

    fn new_string_array<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        array: &'obj_ref Vec<String>,
    ) -> JObjectArray<'local>;
}

impl<'local> JNIEnvExt<'local> for JNIEnv<'local> {
    fn string<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        obj: &'obj_ref JString<'other_local>,
    ) -> String {
        self.get_string(obj).expect("failed to get string").into()
    }

    fn string_array<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        array: &'obj_ref JObjectArray<'other_local>,
    ) -> Vec<String> {
        let length = self
            .get_array_length(array)
            .expect("failed getting array length");
        let mut result = Vec::with_capacity(length as usize);

        for idx in 0..length {
            let java_string = self
                .get_object_array_element(&array, idx)
                .expect("failed getting array element");
            let jstring = unsafe { JString::from_raw(java_string.as_raw()) };
            let string = self.string(&jstring);

            result.push(string);
        }

        result
    }

    fn new_string_array<'other_local: 'obj_ref, 'obj_ref>(
        &mut self,
        strings: &'obj_ref Vec<String>,
    ) -> JObjectArray<'local> {
        let string_class = self
            .find_class("java/lang/String")
            .expect("String does not exist");
        let array = self
            .new_object_array(strings.len() as jint, string_class, JObject::null())
            .expect("failed creating array");

        for (idx, string) in strings.iter().enumerate() {
            let jstring = self.new_string(string).expect("failed creating string");
            self.set_object_array_element(&array, idx as jint, jstring)
                .expect("failed setting array element");
        }

        array
    }
}
