
use actix_web::{HttpServer, App};
use crate::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppData {
    pub jvm:        Arc<jni::JavaVM>,
    pub config:     Config,
}

#[actix_web::main]
pub async fn start(config: Config, jvm: Arc<jni::JavaVM>) -> std::io::Result<()>{
    let port = config.clone().port;
    let data = AppData {
        jvm,
        config,
    };

    crate::jni::log_info(&data.clone().jvm.attach_current_thread().unwrap(), &format!("Starting web server on port {}", port));

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
