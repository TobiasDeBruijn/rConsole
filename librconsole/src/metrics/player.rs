use std::io;
use std::sync::mpsc::Sender;
use crate::jni::{JvmCommand, Method};
use jni::sys::{_jobject, jint};

static mut BUKKIT_CLASS: Option<*mut _jobject> = None;

pub fn get_online_player_count(tx: &Sender<JvmCommand>) -> io::Result<i32> {

    //SAFETY: This is safe because we know that this will never be access from multiple threads, so we can do this
    if unsafe { BUKKIT_CLASS.is_none() } {
        let get_bukkit_class_command = JvmCommand::get_class("org.bukkit.Bukkit");
        let tx_send = tx.send(get_bukkit_class_command.0);
        if tx_send.is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while getting the org.bukkit.Bukkit class"))
        }
        let bukkit_class = get_bukkit_class_command.1.recv().unwrap();

        //SAFETY: This is safe because we know that this will never be access from multiple threads, so we can do this
        unsafe { BUKKIT_CLASS = Some(bukkit_class); }
    }

    //SAFETY: This is safe because we know that this will never be access from multiple threads, so we can do this
    let get_online_players_method = Method::static_method(unsafe { BUKKIT_CLASS.unwrap() }, "getOnlinePlayers", "()Ljava.util.Collection;", Vec::new());
    let get_online_players_command = JvmCommand::exec_method(get_online_players_method);
    let tx_send = tx.send(get_online_players_command.0);
    if tx_send.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while executing the method getOnlinePlayers() on org.bukkit.Bukkit"))
    }
    let online_players_collection = get_online_players_command.1.recv().unwrap();

    let size_method = Method::method(online_players_collection, "size", "()I", Vec::new());
    let size_command = JvmCommand::exec_method(size_method);
    let tx_send = tx.send(size_command.0);
    if tx_send.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while executing the method size() on java.util.Collection"))
    }
    let player_count_jobject = size_command.1.recv().unwrap();

    Ok(player_count_jobject as jint as i32)
}