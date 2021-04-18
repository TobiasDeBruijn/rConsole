use crate::jni::logging::{log_warn, log_debug, log_info, ConsoleLogItem, logging_rec};
use crate::config::Config;
use crate::LogEntry;

use jni::JNIEnv;
use jni::objects::{JString, JClass};
use jni::sys::jlong;
use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    startWebServer(String configFolder)
 * Signature: (Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_startWebServer(env: JNIEnv, _class: JClass, config_folder_jstring: JString) {
    let config_folder: String = env.get_string(config_folder_jstring).expect("Unable to get String from JString 'config_folder_jstring'").into();
    log_info(&env, "Loading library librconsole");

    //Load the configuration file
    let config_wrapped = Config::load(PathBuf::from(config_folder));
    if config_wrapped.is_err() {
        log_warn(&env, "Unable to load configuration file.");
        log_debug(&env, &config_wrapped.err().unwrap());
        return;
    }
    let config = config_wrapped.unwrap();

    log_info(&env, &format!("Loaded librconsole configuration. Listening on 0.0.0.0:{port} and [::]:{port}", port = config.port));

    //Create a Channel for logging purposes
    let (tx , rx): (Sender<ConsoleLogItem>, Receiver<ConsoleLogItem>) = std::sync::mpsc::channel();

    //Start the HTTP server
    std::thread::spawn(move || {
        let _ = crate::webserver::start(config, tx);
    });

    //Start listening for logging 'packets' on the created Receiver Channel
    logging_rec(env, rx);
}

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    appendConsoleLog(String log, long timestamp, String level, String thread)
 * Signature: (Ljava/lang/String;JLjava/lang/String;Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_appendConsoleLog(env: JNIEnv, _class: JClass, log_message_jstring: JString, log_timestamp_jlong: jlong, log_level_jstring: JString, log_thread_jstring: JString) {
    let log_message: String = env.get_string(log_message_jstring).expect("Unable to get String from JString 'log_jstring'").into();
    let log_timestamp: i64 = log_timestamp_jlong;
    let log_level: String = env.get_string(log_level_jstring).expect("Unable to get String from JString 'log_level_jstring").into();
    let log_thread: String = env.get_string(log_thread_jstring).expect("Unable to get String from JString 'log_thread_jstring'").into();

    let log_entry = LogEntry {
        message: log_message,
        timestamp: log_timestamp,
        level: log_level,
        thread: log_thread
    };

    let buffer_pinned = crate::LOG_BUFFER.pin();
    buffer_pinned.insert(buffer_pinned.len() as u32, log_entry);
}