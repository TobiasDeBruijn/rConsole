use jni::JNIEnv;
use std::sync::mpsc::{Receiver, Sender};
use crate::jni::util::str_to_jvalue;

/**
Struct to describe a logging operation to the console
*/
pub struct ConsoleLogItem {
    pub level: LogLevel,
    pub log: String
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
Log to the console

## Parameters
    tx: Reference to the Sender on which to send the log message
    level: The level to log at
    message: The message to log
*/
#[warn(dead_code)]
pub fn log(tx: &Sender<ConsoleLogItem>, level: LogLevel, message: &str) {
    let _ = tx.send(ConsoleLogItem { level, log: message.to_string() });
}

/**
Listen for incoming logging packets on `rx`
**This is a blocking method**

## Parameters
    env: The JNIEnv on which to log
    rx: The Receiver on which ConsoleLogItems will be coming in

*/
pub fn logging_rec(env: JNIEnv, log_rx: Receiver<ConsoleLogItem>) {
    loop {
        let rec = log_rx.recv().unwrap();
        match rec.level {
            LogLevel::Info => log_info(&env, &rec.log),
            LogLevel::Warn => log_warn(&env, &rec.log),
            LogLevel::Debug => log_debug(&env, &rec.log)
        }
    }
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
Log to the INFO [DEBUG] level
*/
pub fn log_debug<'a>(env: &'a JNIEnv, log: &str) {
    /*
    Calls the method logDebug(Object log) in nl.thedutchmc.rconsole.RConsole
    This will only output to the console if IS_DEBUG is true (Java code)
    */
    let log_str_formatted = format!("[librconsole] {}", log);
    let _ = env.call_static_method("nl/thedutchmc/rconsole/RConsole", "logDebug", "(Ljava/lang/Object;)V", &[str_to_jvalue(env, &log_str_formatted)]);
}