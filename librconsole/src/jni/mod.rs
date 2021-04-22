use jni::JNIEnv;
use std::sync::mpsc::Receiver;
use crate::jni::util::str_to_jvalue;

pub mod logging;
pub mod util;

mod web_server_native;

pub fn command_rx(env: JNIEnv, cmd_rx: Receiver<String>) {
    let bukkit_class = env.find_class("org/bukkit/Bukkit").expect("An error occurred while trying to find the Class 'org.bukkit.Bukkit'");
    let console_sender_class = env.find_class("org/bukkit/command/ConsoleCommandSender").expect("An error occurred while trying to find the Class 'org.bukkit.command.ConsoleCommandAppender'");
    let console_sender_object = env.call_static_method(console_sender_class, "getConsoleSender", "()Lorg/bukkit/command/ConsoleCommandSender", &[]).expect("An error occurred while calling the static method 'getConsoleSender' on 'org.bukkit.Bukkit'");

    loop {
        let rec = cmd_rx.recv().unwrap();
        let rec_jstring = str_to_jvalue(&env, &rec);
        env.call_static_method(bukkit_class, "dispatchCommand", "(Lorg/bukkit/command/ConsoleCommandSender;Ljava/lang/String;)Z", &[console_sender_object, rec_jstring]).expect("An error occurred while calling the static method 'dispatchCommand' on 'org.bukkit.Bukkit'");
    }
}
