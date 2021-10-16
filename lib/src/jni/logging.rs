use jni::JNIEnv;
use crate::jni::util::str_to_jvalue;

/**
Struct to describe a logging operation to the console
*/
pub struct ConsoleLogItem {
    pub level: LogLevel,
    pub log: String
}

impl ConsoleLogItem {
    pub fn new(level: LogLevel, log: String) -> ConsoleLogItem {
        ConsoleLogItem {
            level,
            log
        }
    }
}

/**
Enum describing the available console logging levels
*/
#[allow(dead_code)]
pub enum LogLevel {
    Info,
    Warn,
    Debug
}

/**
Log to the INFO level
*/
pub fn log_info<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logInfo(Object log) in nl.thedutchmc.rconsole.RConsole
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("dev/array21/rconsole/core/RConsole", "logInfo", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

/**
Log to the WARN level
*/
pub fn log_warn<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logWarn(Object log) in nl.thedutchmc.rconsole.RConsole
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("dev/array21/rconsole/core/RConsole", "logWarn", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}

/**
Log to the INFO [DEBUG] level
*/
pub fn log_debug<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logDebug(Object log) in nl.thedutchmc.rconsole.RConsole
    This will only output to the console if IS_DEBUG is true (Java code)
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("dev/array21/rconsole/core/RConsole", "logDebug", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}