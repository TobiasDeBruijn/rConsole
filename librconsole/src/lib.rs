#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use flurry::HashMap;
use serde::Serialize;

mod jni;
mod config;
mod webserver;
mod endpoints;

lazy_static! {
    pub static ref LOG_BUFFER: Arc<HashMap<u32, LogEntry>> = Arc::new(flurry::HashMap::new());
}

#[derive(Serialize, Clone)]
pub struct LogEntry {
    pub message:    String,
    pub timestamp:  i64,
    pub level:      String,
    pub thread:     String
}
