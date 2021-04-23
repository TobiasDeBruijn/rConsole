use actix_web::{web, post, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::endpoints::console::CombinedLogEntry;
use crate::jni::logging::{LogLevel, ConsoleLogItem};
use crate::database::Database;
use rusqlite::named_params;
use std::path::PathBuf;
use crate::jni::JvmCommand;

#[derive(Deserialize)]
pub struct LogRequest {
    session_id: String,
    since:      String
}

#[derive(Serialize)]
pub struct LogResponse {
    status:     i16,
    logs:       Option<Vec<CombinedLogEntry>>
}

#[post("/console/new")]
pub async fn post_logs_new(data: web::Data<crate::webserver::AppData>, form: web::Form<LogRequest>) -> HttpResponse {
    let db = Database::new(PathBuf::from(data.db_path.clone())).unwrap();
    let sql_check_session: rusqlite::Result<bool> =  db.connection.query_row_and_then("SELECT EXISTS(SELECT 1 FROM sessions WHERE session_id = :session_id)", named_params! {
        ":session_id": &form.session_id
    }, |row| row.get(0));

    if sql_check_session.is_err() {
        let tx = data.jvm_command_tx.lock().unwrap();
        let jvm_command = JvmCommand::log(ConsoleLogItem::new(LogLevel::Warn,format!("An error occurred while verifying a session_id: {:?}", sql_check_session.err().unwrap()) ));
        tx.send(jvm_command).expect("An issue occurred while sending a JvmCommand");


        return HttpResponse::InternalServerError().finish();
    }

    if !sql_check_session.unwrap() {
        return HttpResponse::Ok().json(LogResponse { status: 401, logs: None});
    }

    let since_wrapped = form.since.parse::<u32>();
    if since_wrapped.is_err() {
        return HttpResponse::BadRequest().body(since_wrapped.unwrap().to_string());
    }
    let since = since_wrapped.unwrap();

    let pinned = crate::LOG_BUFFER.pin();
    if since > ((pinned.len() as u32) -1) {
        return HttpResponse::Ok().json(LogResponse { status: 404, logs: None});
    }

    if since == *pinned.keys().last().unwrap() {
        return HttpResponse::Ok().json(LogResponse { status: 200, logs: None});
    }

    let mut combined_entries: Vec<CombinedLogEntry> = vec![];
    for i in since..pinned.len() as u32 {
        let v = pinned.get(&i).unwrap();
        combined_entries.push(CombinedLogEntry { id: i, log_entry: v.clone() })
    }

    HttpResponse::Ok().json(LogResponse { status: 200, logs: Some(combined_entries)})
}