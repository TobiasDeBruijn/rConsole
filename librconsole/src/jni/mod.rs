use jni::JNIEnv;
use std::sync::mpsc::Receiver;
use crate::jni::util::str_to_jvalue;

pub mod logging;
pub mod util;

mod web_server_native;

pub fn command_rx(env: JNIEnv, cmd_rx: Receiver<String>) {
    let webserver_class = env.find_class("nl/thedutchmc/rconsole/webserver/WebServer").expect("An error occurred while trying to find the Class 'nl.thedutchmc.rconsole.webserver.WebServer");

    loop {
        let rec = cmd_rx.recv().unwrap();
        let rec_jstring = str_to_jvalue(&env, &rec);
        env.call_static_method(webserver_class, "execCommand", "(Ljava/lang/String;)V", &[rec_jstring]).expect("An error occurred while executing a command.");
    }
}
