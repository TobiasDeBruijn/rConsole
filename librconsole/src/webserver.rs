use actix_web::{HttpServer, App};
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use actix_files::Files;
use actix_cors::Cors;
use crate::jni::JvmCommand;

#[derive(Clone)]
pub struct AppData {
    pub jvm_command_tx: Arc<Mutex<Sender<JvmCommand>>>,
    pub config:     Config,
    pub db_path:    String
}

#[actix_web::main]
pub async fn start(config: Config, db_path: String, static_files_path: String, jvm_command_tx: Sender<JvmCommand>) -> std::io::Result<()>{
    let port = config.port;
    let data = AppData {
        jvm_command_tx: Arc::new(Mutex::new(jvm_command_tx)),
        config,
        db_path
    };

    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_method().allow_any_origin().allow_any_header();

        App::new()
            .data(data.clone())
            .wrap(cors)
            .service(crate::endpoints::console::all_logs::post_logs_all)
            .service(crate::endpoints::console::logs_new::post_logs_new)
            .service(crate::endpoints::console::execute_command::post_execute_command)
            .service(crate::endpoints::auth::login::post_login)
            .service(crate::endpoints::stats::cpu::post_get_load_avg)
            .service(crate::endpoints::stats::mem::post_get_mem)
            .service(Files::new("/", &static_files_path)
                .prefer_utf8(true)
                .index_file("index.html")
                .show_files_listing()
            )
    })
    .bind(format!("[::]:{}", port))?
    .run()
    .await
}