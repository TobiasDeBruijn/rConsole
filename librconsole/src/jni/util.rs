use jni::sys::jobjectArray;
use jni::objects::{JValue, JObject};
use jni::JNIEnv;

/**
Convert a bool to a java.lang.Boolean
*/
#[allow(non_snake_case)]
pub fn bool_to_Boolean<'a>(env: &'a JNIEnv, v: bool) -> Result<JValue<'a>, jni::errors::Error> {
    let boolean_class = env.find_class("java/lang/Boolean")?;
    env.call_static_method(boolean_class, "valueOf", "(Z)Ljava/lang/Boolean;", &[JValue::from(v)])
}

/**
Convert a JValue to a JObject
*/
pub fn convert_jvalue_to_jobject(jvalue: JValue) -> Result<JObject, jni::errors::Error> {
    jvalue.l()
}

/**
Create an array of java.lang.String
*/
pub fn create_string_array(env: &JNIEnv, length: usize) -> Result<jobjectArray, jni::errors::Error> {
    let jstring_class = env.find_class("java/lang/String")?;
    let jstring_empty = env.new_string("")?;

    env.new_object_array(length as i32, jstring_class, jstring_empty)
}

/**
Create a java.util.HashMap
*/
pub fn create_hashmap<'a>(env: &'a JNIEnv, length: usize) -> Result<JObject<'a>, jni::errors::Error> {
    let hashmap_jclass = env.find_class("java/util/HashMap")?;
    env.new_object(hashmap_jclass, "(I)V", &[JValue::Int(length as i32)])
}

/**
Insert a <K, V> into a java.util.HashMap
*/
pub fn hashmap_put<'a>(env: &'a JNIEnv, hashmap: JObject<'a>, key: JValue, value: JValue) -> Result<JValue<'a>, jni::errors::Error> {
    env.call_method(hashmap, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", &[key, value])
}

/**
Convert a &str to a JValue
*/
pub fn str_to_jvalue<'a>(env: &'a JNIEnv, str: &str) -> JValue<'a> {
    let log_jstring = env.new_string(str).unwrap();
    let log_jobject = JObject::from(log_jstring);
    let log_jvalue = JValue::from(log_jobject);

    log_jvalue
}