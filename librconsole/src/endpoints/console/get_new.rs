use actix_web::{web, post, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::endpoints::console::CombinedLogEntry;
use crate::jni::logging::{log, LogLevel};

#[derive(Deserialize)]
pub struct LogRequest {
    key:        String,
    name:       String,
    since:      String
}

#[derive(Serialize)]
pub struct LogResponse {
    status:     i16,
    logs:       Option<Vec<CombinedLogEntry>>
}

#[post("/console/since")]
pub async fn post_new(data: web::Data<crate::webserver::AppData>, form: web::Form<LogRequest>) -> HttpResponse {

    //Check if the provided key/name pair exists
    let mut has_valid_key: bool = false;
    for key_item in &data.config.keys {
        if key_item.name == form.name && key_item.key == form.key {
            has_valid_key = true;
        }
    }

    //If the provided key wasn't valid, return a 401
    if !has_valid_key {
        let tx = data.log_tx.lock().unwrap();
        log(&tx, LogLevel::Warn, &format!("A request was received with invalid credentials! The credentials were as follows: name = '{}', key = '{}'", form.name, form.key));

        return HttpResponse::Ok().json(LogResponse { status: 401, logs: None });
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

    let mut combined_entries: Vec<CombinedLogEntry> = vec![];
    for i in since..((pinned.len() as u32) -1) {
        let v = pinned.get(&i).unwrap();
        combined_entries.push(CombinedLogEntry { id: i, log_entry: v.clone() })
    }

    HttpResponse::Ok().json(LogResponse { status: 200, logs: Some(combined_entries)})
}