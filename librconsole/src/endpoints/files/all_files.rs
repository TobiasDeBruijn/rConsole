use actix_web::{HttpResponse, web, post};
use serde::{Serialize, Deserialize};
use crate::webserver::AppData;
use std::path::PathBuf;

#[cfg(windows)]
use regex::Regex;

#[derive(Deserialize)]
pub struct AllFilesRequest {
    session_id: String,
    folder:     Option<String>
}

#[derive(Serialize)]
pub struct AllFilesResponse {
    status: i16,
    files:  Option<Vec<FileSystemEntry>>
}

#[derive(Serialize)]
pub struct FileSystemEntry {
    entry_type: FileSystemEntryType,
    name:       String,
}

#[derive(Serialize)]
pub enum FileSystemEntryType {
    Folder,
    File,
    Unsupported
}

#[post("/files/all")]
pub async fn post_files_all(data: web::Data<AppData>, form: web::Form<AllFilesRequest>) -> HttpResponse {
    let session_id_check = crate::endpoints::check_session_id(&data, &form.session_id);
    if session_id_check.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_id_check.unwrap() {
        return HttpResponse::Ok().json(AllFilesResponse { status: 401, files: None });
    }

    //DB Path is in $serverRoot/plugins/rConsole/librconsole/
    let db_file = PathBuf::from(data.db_path.clone());
    let librconsole_folder = db_file.parent().unwrap();
    let rconsole_folder = librconsole_folder.parent().unwrap();
    let plugins_folder = rconsole_folder.parent().unwrap();
    let server_root_folder = plugins_folder.parent().unwrap();

    let mut root_folder_buf = PathBuf::from(server_root_folder);
    if form.folder.is_some() {
        //It only needs to be mutable on Windows
        #[allow(unused_mut)]
        let mut path = form.folder.clone().unwrap();
        
        //We definitely don't want people trying to access directories outside the server directory
        if path.contains("..") {
            return HttpResponse::Ok().json(AllFilesResponse { status: 403, files: None });
        }

        //Dont want people accessing the server's root
        if path.starts_with("/") {
            return HttpResponse::Ok().json(AllFilesResponse { status: 403, files: None });
        }

        #[cfg(windows)]
        {
            if path.starts_with(r#"\"#) {
                return HttpResponse::Ok().json(AllFilesResponse { status: 403, files: None });
            }

            path = path.replace("/", r#"\"#);

            //Check if the path starts with e.g C:\\ or C:\
            let root_drive_regex = Regex::new(r#".:"#).unwrap();
            if root_drive_regex.is_match(path.as_str()) {
                return HttpResponse::Ok().json(AllFilesResponse { status: 403, files: None });
            }
        }

        root_folder_buf.push(PathBuf::from(path).as_path());
    }

    let paths = std::fs::read_dir(root_folder_buf.as_path());
    if paths.is_err() {
        return HttpResponse::Ok().json(AllFilesResponse { status: 404, files: None });
    }

    let mut fs_entries: Vec<FileSystemEntry> = Vec::new();
    for path_wrapped in paths.unwrap() {
        let path = path_wrapped.unwrap();

        let fs_entry = FileSystemEntry {
            name: path.file_name().to_str().unwrap().to_string(),
            entry_type: {
                let file_type = path.file_type().unwrap();
                let fs_type: FileSystemEntryType;
                if file_type.is_dir() {
                    fs_type = FileSystemEntryType::Folder;
                } else if file_type.is_file() {
                    fs_type = FileSystemEntryType::File;
                } else {
                    fs_type = FileSystemEntryType::Unsupported;
                }

                fs_type
            }
        };

        fs_entries.push(fs_entry);
    }

    HttpResponse::Ok().json(AllFilesResponse { status: 200, files: Some(fs_entries)})
}