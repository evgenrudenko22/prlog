mod core;
pub mod formatter;
mod writer;
pub mod entry;

pub use core::{PrLogger, Config};

use log::{SetLoggerError};

pub fn init(config: Config) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(PrLogger::new(config.clone())))
        .map(|()| log::set_max_level(config.max_log_level().to_level_filter()))
}