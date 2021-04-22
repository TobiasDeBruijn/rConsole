#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use flurry::HashMap;
use serde::Serialize;
use crate::database::Database;
use std::cell::Cell;
use crate::config::Config;
use std::sync::mpsc::Receiver;

mod jni;
mod config;
mod webserver;
mod endpoints;
mod database;

lazy_static! {
    pub static ref LOG_BUFFER: Arc<HashMap<u32, LogEntry>> = Arc::new(flurry::HashMap::new());
    pub static ref DATABASE: Arc<Mutex<Cell<Option<Database>>>> = Arc::new(Mutex::new(Cell::new(None)));
    pub static ref CONFIG: Arc<Mutex<Cell<Option<Config>>>> = Arc::new(Mutex::new(Cell::new(None)));
    pub static ref RX_COMMANDS: Arc<Mutex<Cell<Option<Receiver<String>>>>> = Arc::new(Mutex::new(Cell::new(None)));
}

#[derive(Serialize, Clone)]
pub struct LogEntry {
    pub message:        String,
    pub timestamp:      i64,
    pub level:          String,
    pub thread:         String,
    pub log_attributes: Vec<LogAttribute>
}

#[derive(Serialize, Clone)]
pub enum LogAttribute {
    LogNoIndex
}
