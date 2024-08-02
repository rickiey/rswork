use log::{debug, error, info, trace, warn};
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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, _message, record| {
            let log_entry = LogEntry {
                ts: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                level: record.level().to_string(),
                line:record.file().unwrap().to_string() +":"+record.line().unwrap().to_string().as_str(),
                message: record.args().to_string(),
            };
            let json_value = serde_json::to_string(&log_entry);
            out.finish(format_args!("{}",json_value.unwrap_or("serder err".to_string())));
            // out.finish(format_args!(
            //     "{} {} {}:{} {}",
            //     chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            //     record.level(),
            //     record.file().unwrap_or("unknow"),
            //     record.line().unwrap_or(0),
            //     message
            // ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        // .chain(fern::log_file("log/output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    trace!("Hello, world!");
    debug!("Hello, world!");
    info!("Hello, world!");
    warn!("Warning!");
    error!("Now exiting.");

    Ok(())
}