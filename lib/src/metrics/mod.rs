use crate::jni::logging::{ConsoleLogItem, LogLevel};
use crate::jni::JvmCommand;
use std::sync::mpsc::Sender;
use serde::Serialize;
use flurry::HashMap;

mod cpu;
mod mem;
mod player;

///How often should we 'take' a metrics reading, in seconds
const METRIC_INTERVAL_SECS: u8 = 10;

///For how long should metrics readings be stored, in minutes
const LOG_RETENTION_TIME_MIN: u8 = 2;

lazy_static! {
    pub static ref METRICS: HashMap<i64, MetricsCollection> = HashMap::new();
}

#[derive(Clone, Serialize)]
pub struct MetricsCollection {
    pub cpu:    CpuMetric,
    pub mem:    MemoryMetric,
    pub player: i32
}

#[derive(Clone, Serialize)]
pub struct CpuMetric {
    pub load_avg:   f64
}

#[derive(Clone, Serialize)]
pub struct MemoryMetric {
    pub max_mem:    f64,
    pub total_mem:  f64,
    pub free_mem:   f64
}

pub fn collect_metrics(jvm_tx: Sender<JvmCommand>) {
    std::thread::spawn(|| {
        metrics_garbage_collector();
    });

    loop {
        //Cpu metrics
        let cpu_load_avg = cpu::get_load_avg();
        if cpu_load_avg.is_err() {
            log_warn(&jvm_tx, &format!("An error occurred while fetching CPU Load average metrics: {:?}", cpu_load_avg.err().unwrap()));
            continue;
        }

        //Memory metrics
        let mem_usage_wrapped = mem::get_mem_usage(&jvm_tx);
        if mem_usage_wrapped.is_err() {
            log_warn(&jvm_tx, &format!("An error occurred while fetching Memory usage metrics: {:?}", mem_usage_wrapped.err().unwrap()));
            continue;
        }
        let mem_usage = mem_usage_wrapped.unwrap();

        let online_players = player::get_online_player_count(&jvm_tx);
        if online_players.is_err() {
            log_warn(&jvm_tx, &format!("An error occurred while fetching online Players metrics: {:?}", online_players.err().unwrap()));
            continue;
        }

        let metrics_collection = MetricsCollection {
            cpu: CpuMetric {
                //Round to three decimal digits
                load_avg: (cpu_load_avg.unwrap() * 1_000f64).round() / 1_000f64
            },
            mem: MemoryMetric {
                //Round to two decimal digits
                total_mem: (mem_usage.0 * 100f64).round() / 100f64,
                free_mem: (mem_usage.1 * 100f64).round() / 100f64,
                max_mem: (mem_usage.2 * 100f64).round() / 1000f64
            },
            player: online_players.unwrap()
        };

        let epoch_now = chrono::Utc::now().timestamp();
        METRICS.pin().insert(epoch_now, metrics_collection);

        std::thread::sleep(std::time::Duration::from_secs(METRIC_INTERVAL_SECS as u64));
    }
}

fn metrics_garbage_collector() {
    loop {
        let epoch_retention_ago = (chrono::Utc::now() - chrono::Duration::minutes(LOG_RETENTION_TIME_MIN as i64)).timestamp();
        METRICS.pin().retain_force(|k, _v| *k >= epoch_retention_ago);
        std::thread::sleep(std::time::Duration::from_secs(LOG_RETENTION_TIME_MIN as u64 * 60u64));
    }
}

fn log_warn(tx: &Sender<JvmCommand>, log: &str) {
    let log_cmd = JvmCommand::log(ConsoleLogItem { level: LogLevel::Warn, log: log.to_string()});
    tx.send(log_cmd).expect(&format!("An issue occurred while logging '{}'", log));
}