use std::env::set_var;
use log::{debug, error, log_enabled, info, trace,warn, Level};
use std::io::Write;
use env_logger::Env;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct LogEntry {
    ts: String,
    level: String,
    line: String,
    message: String,
}

fn main() {
    set_var("RUST_LOG", "trace");


    // env_logger::init();
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("warn"));
    // env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    builder.format(|buf, record|{
        let log_entry = LogEntry {
            ts: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: record.level().to_string(),
            line:record.file().unwrap().to_string() +":"+record.line().unwrap().to_string().as_str(),
            message: record.args().to_string(),
        };
        let json_value = serde_json::to_string(&log_entry);
        writeln!(buf, "{}",json_value.unwrap())
    } ).init();

    
    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }

    let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now();
    trace!("{}",utc.to_rfc3339_opts(SecondsFormat::Secs, false));
    debug!("{}",local.format("%Y-%m-%d %H:%M:%S").to_string());
    info!("{}",local.format("%Y-%m-%d %H:%M:%S").to_string());
    warn!("{}",local.to_rfc3339_opts(SecondsFormat::Secs, false));
    warn!("{}",local.to_rfc3339_opts(SecondsFormat::Secs, true));
    error!("{}",local.to_string());
}