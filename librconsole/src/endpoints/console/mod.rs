use crate::LogEntry;
use serde::Serialize;

pub mod all_logs;
pub mod logs_new;

#[derive(Serialize)]
pub struct CombinedLogEntry {
    id: u32,
    log_entry: LogEntry
}