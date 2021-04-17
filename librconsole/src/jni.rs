use jni::JNIEnv;
use jni::objects::{JClass, JString, JValue, JObject};
use crate::config::Config;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn log_info<'a>(env: &'a JNIEnv, log: &str) {
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logInfo", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

pub fn log_warn<'a>(env: &'a JNIEnv, log: &str) {
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logWarn", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

pub fn log_debug<'a>(env: &'a JNIEnv, log: &str) {
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logDebug", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

fn str_to_jvalue<'a>(env: &'a JNIEnv, str: &str) -> JValue<'a> {
    let log_jstring = env.new_string(str).unwrap();
    let log_jobject = JObject::from(log_jstring);
    let log_jvalue = JValue::from(log_jobject);

    log_jvalue
}

#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_dashboard_Native_nativeStartDashboardServer(env: JNIEnv, object: JObject, config_folder_jstring: JString) {
    let config_folder: String = env.get_string(config_folder_jstring).expect("Unable to get String from JString 'config_folder_jstring'").into();

    log_info(&env, "Loading library librconsole");

    let config_wrapped = Config::load(PathBuf::from(config_folder));
    if config_wrapped.is_err() {
        log_warn(&env, "Unable to load configuration file.");
        log_debug(&env, &config_wrapped.err().unwrap());
        return;
    }
    let config = config_wrapped.unwrap();

    let array_list_class = env.find_class("java/util/ArrayList").unwrap();
    let array_list_constructor = env.get_method_id(array_list_class, "<init>", "(I)V");

    env.call_method(object, "setLogBuffer", "([Ljava/lang/String;)V", &[jval]);

    let jvm = Arc::new(env.get_java_vm().unwrap());
    let _ = crate::webserver::start(config, jvm);
}

#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_dashboard_Native_nativeStopDashboardServer(_env: JNIEnv, _object: JObject) {

}

#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_dashboard_Native_appendConsoleLog(env: JNIEnv, object: JObject, log_jstring: JString) {
    let log: String = env.get_string(log_jstring).expect("Unable to get String from JString 'log_jstring'").into();
    log_buffer.get_mut().unwrap().push(log);
}