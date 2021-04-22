
use actix_web::{HttpServer, App};
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::jni::logging::ConsoleLogItem;
use actix_files::Files;
use actix_cors::Cors;

#[derive(Clone)]
pub struct AppData {
    pub log_tx:     Arc<Mutex<Sender<ConsoleLogItem>>>,
    pub command_tx: Arc<Mutex<Sender<String>>>,
    pub config:     Config,
    pub db_path:    String
}

#[actix_web::main]
pub async fn start(config: Config, log_tx: Sender<ConsoleLogItem>, command_tx: Sender<String>, db_path: String, static_files_path: String) -> std::io::Result<()>{
    let port = config.port;
    let data = AppData {
        log_tx: Arc::new(Mutex::new(log_tx)),
        command_tx: Arc::new(Mutex::new(command_tx)),
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
            .service(crate::endpoints::auth::login::post_login)
            .service(Files::new("/", &static_files_path)
                .prefer_utf8(true)
                .index_file("index.html")
                .show_files_listing()
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .bind(format!("[::]:{}", port))?
    .run()
    .await
}