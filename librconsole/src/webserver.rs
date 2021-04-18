
use actix_web::{HttpServer, App};
use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::jni::logging::ConsoleLogItem;

#[derive(Clone)]
pub struct AppData {
    pub config:     Config,
    pub log_tx:     Arc<Mutex<Sender<ConsoleLogItem>>>
}

#[actix_web::main]
pub async fn start(config: Config, tx: Sender<ConsoleLogItem>) -> std::io::Result<()>{
    let port = config.clone().port;
    let data = AppData {
        config,
        log_tx: Arc::new(Mutex::new(tx))
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(crate::endpoints::console::get_all_logs::post_all)
            .service(crate::endpoints::console::get_new::post_new)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .bind(format!("[::]:{}", port))?
    .run()
    .await
}