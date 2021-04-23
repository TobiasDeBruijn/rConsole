use crate::jni::logging::{LogLevel, ConsoleLogItem};
use crate::database::Database;
use crate::webserver::AppData;

use actix_web::{web, post, HttpResponse};
use serde::{Serialize, Deserialize};
use rusqlite::named_params;
use std::path::PathBuf;
use crate::jni::{JvmCommand, Method, Argument};

#[derive(Deserialize)]
pub struct ExecuteCommandRequest {
    session_id: String,
    command:    String
}

#[derive(Serialize)]
pub struct ExecuteCommandResponse {
    status: i16
}

#[post("/console/command")]
pub async fn post_execute_command(data: web::Data<AppData>, form: web::Form<ExecuteCommandRequest>) -> HttpResponse {
    let db = Database::new(PathBuf::from(data.db_path.clone())).unwrap();
    let sql_check_session: rusqlite::Result<bool> =  db.connection.query_row("SELECT EXISTS(SELECT 1 FROM sessions WHERE session_id = :session_id)", named_params! {
        ":session_id": &form.session_id
    }, |row| row.get(0));

    let jvm_command_tx = data.jvm_command_tx.lock().unwrap();

    if sql_check_session.is_err() {
        let jvm_command = JvmCommand::log(ConsoleLogItem::new(LogLevel::Warn,format!("An error occurred while verifying a session_id: {:?}", sql_check_session.err().unwrap()) ));
        &jvm_command_tx.send(jvm_command);

        return HttpResponse::InternalServerError().finish();
    }

    if !sql_check_session.unwrap() {
        return HttpResponse::Ok().json(ExecuteCommandResponse { status: 401});
    }

    //Get the class 'nl.thedutchmc.rconsole.webserver.WebServer
    let jvm_command_get_webserver_class = JvmCommand::get_class("nl/thedutchmc/rconsole/webserver/WebServer");
    jvm_command_tx.send(jvm_command_get_webserver_class.0).expect("An issue occurred while sending a JvmCommand");
    let jvm_response = jvm_command_get_webserver_class.1.recv().expect("An issue occurred while reading the response of a JvmCommand execution");

    //Execute the method 'nl.thedutchmc.rconsole.webserver.WebServer#execCommand(String)'
    let exec_command_method = Method::static_method(jvm_response, "execCommand", "(Ljava/lang/String;)V", vec![Argument::String(form.command.clone())]);
    let jvm_command_exec_command = JvmCommand::exec_method_no_return(exec_command_method);
    jvm_command_tx.send(jvm_command_exec_command).expect("An issue occurred while sending a JvmCommand");

    HttpResponse::Ok().json(ExecuteCommandResponse { status: 200 })
}