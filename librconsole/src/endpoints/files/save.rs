use crate::jni::logging::{ConsoleLogItem, LogLevel};
use crate::webserver::AppData;
use crate::jni::JvmCommand;
use actix_web::{HttpResponse, post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::io::Write;

#[cfg(windows)]
use regex::Regex;

#[derive(Deserialize)]
pub struct SaveFileRequest {
    session_id: String,
    filename:   String,
    content:    String
}

#[derive(Serialize)]
pub struct SaveFileResponse {
    status:     i16,
}

#[post("/files/save")]
pub async fn post_files_save(data: web::Data<AppData>, form: web::Form<SaveFileRequest>) -> HttpResponse {
    let session_id_check = crate::endpoints::check_session_id(&data, &form.session_id);
    if session_id_check.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_id_check.unwrap() {
        return HttpResponse::Ok().json(SaveFileResponse { status: 401 });
    }

    //DB Path is in $serverRoot/plugins/rConsole/librconsole/
    let db_file = PathBuf::from(data.db_path.clone());
    let librconsole_folder = db_file.parent().unwrap();
    let rconsole_folder = librconsole_folder.parent().unwrap();
    let plugins_folder = rconsole_folder.parent().unwrap();
    let server_root_folder = plugins_folder.parent().unwrap();

    let mut file_buf = PathBuf::from(server_root_folder);
    //It only needs to be mutable on Windows
    #[allow(unused_mut)]
    let mut path = form.filename.clone();

    //We definitely don't want people trying to access directories outside the server directory
    if path.contains("..") {
        return HttpResponse::Ok().json(SaveFileResponse { status: 403 });
    }

    //Don't want people accessing the server's root
    if path.starts_with("/") {
        return HttpResponse::Ok().json(SaveFileResponse { status: 403 });
    }

    #[cfg(windows)]
    {
        if path.starts_with(r#"\"#) {
            return HttpResponse::Ok().json(SaveFileResponse { status: 403 });
        }

        path = path.replace("/", r#"\"#);

        //Check if the path starts with e.g C:\\ or C:\
        let root_drive_regex = Regex::new(r#".:"#).unwrap();
        if root_drive_regex.is_match(path.as_str()) {
            return HttpResponse::Ok().json(SaveFileResponse { status: 403 });
        }
    }

    file_buf.push(PathBuf::from(path).as_path());

    let file = if !file_buf.exists() {
        std::fs::File::create(file_buf.as_path())
    } else {
        std::fs::File::open(file_buf.as_path())
    };

    if file.is_err() {
        let tx = data.jvm_command_tx.lock().unwrap();
        let jvm_command = JvmCommand::log(ConsoleLogItem::new(LogLevel::Warn,format!("An error occurred while opening a file: {:?}", file.err().unwrap()) ));
        tx.send(jvm_command).expect("An issue occurred while sending a JvmCommand");

        return HttpResponse::InternalServerError().finish();
    }

    let write_result = file.unwrap().write_all(form.content.as_bytes());
    if write_result.is_err() {
        let tx = data.jvm_command_tx.lock().unwrap();
        let jvm_command = JvmCommand::log(ConsoleLogItem::new(LogLevel::Warn,format!("An error occurred while writing to a file: {:?}", write_result.err().unwrap()) ));
        tx.send(jvm_command).expect("An issue occurred while sending a JvmCommand");

        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(SaveFileResponse { status: 200 })
}