//! # prlog
//!
//! `prlog` is a lightweight and modular logging system for Rust.
//! The main feature is dual output:
//! 1. **Console** — for quick tracking of events in real time (with colors).
//! 2. **File** — to save history in `level|timestamp|target|message` format.
//!
//! The library comes with the `prlog-cli` CLI utility for easy viewing
mod core;
pub mod formatter;
#[cfg(feature = "file-log")]
mod writer;
pub mod entry;

pub use core::{PrLogger, Config};

use log::{SetLoggerError};

/// Initialize global logger with config
///
/// # Params
/// * `config` - `Config` used to initialize logger
///
/// # Examples
/// ## Init with default config
/// ```
/// use prlog::Config;
/// prlog::init(Config::default()).unwrap()
/// ```
/// ## Init with max log level set to Warn
/// ```
/// use log::Level;
/// use prlog::Config;
/// prlog::init(Config::default().set_max_log_level(Level::Warn)).unwrap()
/// ```
pub fn init(config: Config) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(PrLogger::new(config.clone())))
        .map(|()| log::set_max_level(config.max_log_level().to_level_filter()))
}