
use actix_web::{HttpServer, App};
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::jni::logging::ConsoleLogItem;

#[derive(Clone)]
pub struct AppData {
    pub log_tx:     Arc<Mutex<Sender<ConsoleLogItem>>>,
    pub config:     Config,
    pub db_path:    String
}

#[actix_web::main]
pub async fn start(config: Config, tx: Sender<ConsoleLogItem>, db_path: String) -> std::io::Result<()>{
    let port = config.port;
    let data = AppData {
        log_tx: Arc::new(Mutex::new(tx)),
        config,
        db_path
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(crate::endpoints::console::all_logs::post_logs_all)
            .service(crate::endpoints::console::logs_since::post_logs_since)
            .service(crate::endpoints::auth::login::post_login)

    })
    .bind(format!("0.0.0.0:{}", port))?
    .bind(format!("[::]:{}", port))?
    .run()
    .await
}