use crate::webserver::AppData;
use actix_web::{HttpResponse, post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[cfg(windows)]
use regex::Regex;

#[derive(Deserialize)]
pub struct FileRequest {
    session_id: String,
    filename:   String
}

#[derive(Serialize)]
pub struct FileResponse {
    status:     i16,
    content:    Option<String>
}

#[post("/files/file")]
pub async fn post_files_file(data: web::Data<AppData>, form: web::Form<FileRequest>) -> HttpResponse {
    let session_id_check = crate::endpoints::check_session_id(&data, &form.session_id);
    if session_id_check.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_id_check.unwrap() {
        return HttpResponse::Ok().json(FileResponse { status: 401, content: None });
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
        return HttpResponse::Ok().json(FileResponse { status: 403, content: None });
    }

    //Don't want people accessing the server's root
    if path.starts_with("/") {
        return HttpResponse::Ok().json(FileResponse { status: 403, content: None });
    }

    #[cfg(windows)]
        {
        if path.starts_with(r#"\"#) {
            return HttpResponse::Ok().json(FileResponse { status: 403, content: None });
        }

        path = path.replace("/", r#"\"#);

        //Check if the path starts with e.g C:\\ or C:\
        let root_drive_regex = Regex::new(r#".:"#).unwrap();
        if root_drive_regex.is_match(path.as_str()) {
            return HttpResponse::Ok().json(FileResponse { status: 403, content: None });
        }
    }

    file_buf.push(PathBuf::from(path).as_path());

    if !file_buf.exists() {
        return HttpResponse::Ok().json(FileResponse { status: 404, content: None });
    }

    let file_content = std::fs::read_to_string(file_buf.as_path());
    if file_content.is_err() {
        return HttpResponse::Ok().json(FileResponse { status: 415, content: None });
    }

    HttpResponse::Ok().json(FileResponse { status: 200, content: Some(file_content.unwrap()) })
}