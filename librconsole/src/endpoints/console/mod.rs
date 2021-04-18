use crate::LogEntry;
use serde::Serialize;

pub mod get_all_logs;
pub mod get_new;

#[derive(Serialize)]
pub struct CombinedLogEntry {
    id:         u32,
    log_entry:  LogEntry
}