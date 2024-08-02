use tracing::{info,Level};
use tracing_subscriber::EnvFilter;


pub fn configure(log_level_file: &str, log_directory: &str) {
    let file_appender = tracing_appender::rolling::daily(log_directory, "zgs.log");
    let (non_blocking, _) = tracing_appender::non_blocking(file_appender);
    let builder = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::default())
        .with_writer(non_blocking)
        .with_ansi(false)
        // .with_file(true)
        // .with_line_number(true)
        // .with_thread_names(true)
        .with_filter_reloading();

    let handle = builder.reload_handle();
    builder.init();

    let level_file = log_level_file.trim_end().trim_end_matches(|c| c == '\n' || c == '\r' || c == '\t').to_string();

    // load config synchronously
    let  config = std::fs::read_to_string(&level_file).unwrap_or_default();
    println!("{}\n",config);
    let _ = handle.reload(&config);

}
fn main() {
    // tracing_subscriber::fmt::init();
    configure("log_config", "log");
    info!(test="test","This will _not_ be logged to stdout");
    let original_string = "Hello, world!\n\t";
    let trimmed_string = original_string.trim_end_matches(|c: char| c.is_whitespace());
    println!("Original: '{}'", original_string);
    println!("Trimmed: '{}'", trimmed_string);
}