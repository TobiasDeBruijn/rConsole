use crate::jni::JvmCommand;
use crate::webserver::AppData;
use crate::database::Database;
use crate::jni::logging::{ConsoleLogItem, LogLevel};
use rusqlite::named_params;
use std::io;
use std::path::PathBuf;

pub mod console;
pub mod auth;
pub mod stats;

pub fn check_session_id(data: &AppData, session_id: &str) -> io::Result<bool> {
    let db = Database::new(PathBuf::from(data.db_path.clone())).unwrap();
    let sql_check_session: rusqlite::Result<bool> =  db.connection.query_row("SELECT EXISTS(SELECT 1 FROM sessions WHERE session_id = :session_id)", named_params! {
        ":session_id": session_id
    }, |row| row.get(0));

    if sql_check_session.is_err() {
        let tx = data.jvm_command_tx.lock().unwrap();
        let jvm_command = JvmCommand::log(ConsoleLogItem::new(LogLevel::Warn,format!("An error occurred while verifying a session_id: {:?}", sql_check_session.err().unwrap()) ));
        tx.send(jvm_command).expect("An issue occurred while sending a JvmCommand");

        return Err(io::Error::new(io::ErrorKind::Other, "An issue occurred while executing the database query"));
    }

    Ok(sql_check_session.unwrap())
}