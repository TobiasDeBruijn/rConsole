use jni::JNIEnv;
use jni::objects::{JString, JValue, JObject, JClass};
use crate::config::Config;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

lazy_static! {
    pub static ref LOG_BUFFER: Arc<Mutex<RefCell<Vec<String>>>> = Arc::new(Mutex::new(RefCell::new(Vec::new())));
}

/**
Log to the INFO level
*/
pub fn log_info<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logInfo(Object log) in nl.thedutchmc.rconsole.RConsole
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logInfo", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

/**
Log to the WARN level
*/
pub fn log_warn<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logWarn(Object log) in nl.thedutchmc.rconsole.RConsole
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logWarn", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

/**
Log to the DEBUG level
*/
pub fn log_debug<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logDebug(Object log) in nl.thedutchmc.rconsole.RConsole
    This will only output to the console if IS_DEBUG is set (Java code)
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logDebug", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

/**
Convert a &str to a JValue
*/
fn str_to_jvalue<'a>(env: &'a JNIEnv, str: &str) -> JValue<'a> {
    let log_jstring = env.new_string(str).unwrap();
    let log_jobject = JObject::from(log_jstring);
    let log_jvalue = JValue::from(log_jobject);

    log_jvalue
}

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    startWebServer
 * Signature: (Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_startWebServer(env: JNIEnv, _class: JClass, config_folder_jstring: JString) {
    let config_folder: String = env.get_string(config_folder_jstring).expect("Unable to get String from JString 'config_folder_jstring'").into();

    log_info(&env, "Loading library librconsole");

    let config_wrapped = Config::load(PathBuf::from(config_folder));
    if config_wrapped.is_err() {
        log_warn(&env, "Unable to load configuration file.");
        log_debug(&env, &config_wrapped.err().unwrap());
        return;
    }
    let config = config_wrapped.unwrap();

    let jvm = Arc::new(env.get_java_vm().unwrap());
    let _ = crate::webserver::start(config, jvm);
}

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    appendConsoleLog
 * Signature: (Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_appendConsoleLog(env: JNIEnv, _class: JClass, log_jstring: JString) {
    let log: String = env.get_string(log_jstring).expect("Unable to get String from JString 'log_jstring'").into();

    let n = LOG_BUFFER.lock().unwrap();
    n.take().push(log);

    for i in n.take().iter() {
        log_info(&env, i);
    }
}