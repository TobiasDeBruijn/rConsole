use actix_web::{HttpResponse, web, post};
use serde::{Serialize, Deserialize};
use crate::jni::logging::{LogLevel, log};
use crate::endpoints::console::CombinedLogEntry;

#[derive(Deserialize)]
pub struct LogRequest {
    key:        String,
    name:       String
}

#[derive(Serialize)]
pub struct LogResponse {
    status:     i16,
    logs:       Option<Vec<CombinedLogEntry>>
}

#[post("/console/all")]
pub async fn post_all(data: web::Data<crate::webserver::AppData>, form: web::Form<LogRequest>) -> HttpResponse {

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

    //Iterate over the LOG_BUFFER map to construct a Vec<CombinedLogEntry>
    let mut combined_entries: Vec<CombinedLogEntry> = vec![];
    let buffer_pinned = crate::LOG_BUFFER.pin();
    for i in 0..(buffer_pinned.len() -1){
        let (k, v) = buffer_pinned.get_key_value(&(i as u32)).unwrap();
        combined_entries.push(CombinedLogEntry { id: *k, log_entry: v.clone() })
    }

    //Return the results
    return HttpResponse::Ok().json(LogResponse { status: 200, logs: Some(combined_entries) })
}