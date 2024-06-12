// src/utils/logger.rs

use fern::Dispatch;
use chrono::Local;
use std::env;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let env = env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string());
    let is_development = env == "development";

    let base_config = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(if is_development { log::LevelFilter::Debug } else { log::LevelFilter::Warn });

    let console_config = Dispatch::new()
        .chain(std::io::stdout());

    let file_error_config = Dispatch::new()
        .level(log::LevelFilter::Error)
        .chain(fern::log_file("/tmp/ngrok-agent-rust-error.log")?);

    let file_combined_config = Dispatch::new()
        .chain(fern::log_file("/tmp/ngrok-agent-rust-combined.log")?);

    if is_development {
        base_config
            .chain(console_config)
            .apply()?;
    } else {
        base_config
            .chain(file_error_config)
            .chain(file_combined_config)
            .apply()?;
    }

    Ok(())
}
