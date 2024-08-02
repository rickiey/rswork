use std::{path::Path, time::Duration};
use serde::Deserialize;
use std::sync::mpsc::channel;
use std::fs;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

#[derive(Debug, Deserialize)]
struct ConfigData {
    #[serde(default = "default_key1")]
    key1: String,
    #[serde(default = "default_key2")]
    key2: i32,
}

fn default_key1() -> String {
    "default_value1".to_string()
}

fn default_key2() -> i32 {
    42
}

fn load_config(path: &Path) -> ConfigData {
    let config_data = fs::read_to_string(path).expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("JSON was not well-formatted")
}

/// Example for debouncer mini
fn main() {

    let p = Path::new("watch_file/src/config/config.json");
    // setup debouncer
    let (tx, rx) = channel();

    // No specific tickrate, max debounce time 1 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx).unwrap();

    debouncer
        .watcher()
        .watch(p, RecursiveMode::NonRecursive)
        .unwrap();

    let mut d = load_config(p);
    println!("配置文件 {:?} ", d);
    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => events
                .iter()
                .for_each(|event| {
                    if event.kind == DebouncedEventKind::AnyContinuous {
                        return;
                    }
                    println!("Event {event:?}");
                    d = load_config(p);
                    println!("新配置文件 {:?} ", d);

                }),
            Err(error) => println!("Error {error:?}"),
        }
    }
}