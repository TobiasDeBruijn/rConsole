use crate::jni::logging::{LogLevel, log};
use crate::database::Database;
use crate::webserver::AppData;

use actix_web::{web, post, HttpResponse};
use serde::{Serialize, Deserialize};
use rusqlite::named_params;
use std::path::PathBuf;

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

    if sql_check_session.is_err() {
        let tx = data.log_tx.lock().unwrap();
        log(&tx, LogLevel::Warn, &format!("An error occurred while verifying a session_id: {:?}", sql_check_session.err().unwrap()));

        return HttpResponse::InternalServerError().finish();
    }

    if !sql_check_session.unwrap() {
        return HttpResponse::Ok().json(ExecuteCommandResponse { status: 401});
    }

    let command_tx = data.command_tx.lock().unwrap();
    command_tx.send(form.command.clone()).expect("An issue occurred while sending a command");

    HttpResponse::Ok().json(ExecuteCommandResponse { status: 200 })
}