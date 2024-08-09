use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
// use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
// use core::time;
use std::{thread, time};
// use std::t;

pub fn config_log(log_config_file: &str, log_file: &str, log_directory: &str) {
    let file_appender = tracing_appender::rolling::daily(log_directory, log_file);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let builder = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::default())
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_filter_reloading();
    // .init();

    let level_file = log_config_file.trim_end().to_string();

    // load config synchronously
    let mut config = std::fs::read_to_string(&level_file)
        .unwrap_or_default()
        .trim_end()
        .to_string();
    println!("log config: {}\n", config);

    let handle = builder.reload_handle();

    let _ = handle.reload(&config).expect("reload failed: ");
    // subscriber::set_global_default(builder).expect("set golbal err:");
    builder.init();
    info!(test = "init", "This will _not_ be logged to stdout");
    // periodically check for config changes
    thread::spawn(move || {
        let _moved_guard = guard;
        // let  config = std::fs::read_to_string(&level_file).unwrap_or_default().trim_end().to_string();
        loop {
            thread::sleep(time::Duration::from_secs(3));

            let new_config: String = match std::fs::read_to_string(&level_file) {
                Ok(c) => {
                    let nc = c.trim_end().to_string();
                    if nc == config {
                        continue;
                    } else {
                        nc
                    }
                }
                Err(e) => {
                    println!("Unable to read log file {}: {:?}", level_file, e);
                    continue;
                }
            };

            println!("Updating log config to {:?}", new_config);

            match handle.reload(&new_config) {
                Ok(()) => config = new_config,
                Err(e) => {
                    println!("Failed to load new config: {:?}", e);
                }
            }
        }
    });
    // move the log writer guard so that it's not dropped.
    // guard
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "debug");

    // tracing_subscriber::fmt::init();
    // configure("log_config", "log");
    let log_directory = "log";
    let log_file = "zgs.log";
    // let _guard =  config_log("log_config",log_file,log_directory);
    config_log("log_config", log_file, log_directory);

    info!(test = "test", "This will _not_ be logged to stdout");

    let number_of_teams: i32 = 3;
    info!(number_of_teams, "We've got {} teams!", number_of_teams);
    invoked_tracing();
    thread::sleep(time::Duration::from_secs(160));
}

fn invoked_tracing() {
    info!("this is sub function");
}
