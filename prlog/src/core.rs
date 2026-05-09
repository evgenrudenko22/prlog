use std::fs::File;
use std::path::PathBuf;
use log::{Level, Metadata, Record};
use crate::entry::LogEntry;
use crate::{formatter, writer};

pub struct PrLogger {
    config: Config
}

impl PrLogger {
    pub fn new(config: Config) -> PrLogger {
        File::create(&config.save_path).unwrap();
        PrLogger { config }
    }
}

impl log::Log for PrLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.config.max_log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = chrono::Local::now();
            let time_str = now.format(&self.config.time_format).to_string();

            let level = record.level().to_string();
            let target = record.target();
            let message = format!("{}", record.args());

            let console_out = formatter::format_console(&level, &time_str, target, &message);
            println!("{}", console_out);

            let entry = LogEntry::new(level,
                                      target.to_string(),
                                      now.to_string(),
                                      message);
            writer::write(self.config.save_path(), &entry);
        }
    }

    fn flush(&self) {}
}

#[derive(Debug, Clone)]
pub struct Config {
    max_log_level: Level,
    save_path: PathBuf,
    time_format: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_log_level: Level::Debug,
            save_path: PathBuf::from("log.txt"),
            time_format: "%H:%M:%S".to_string(),
        }
    }

    pub fn max_log_level(&self) -> Level {
        self.max_log_level
    }

    pub fn save_path(&self) -> PathBuf {
        self.save_path.clone()
    }

    pub fn time_format(&self) -> String {
        self.time_format.clone()
    }

    pub fn set_max_log_level(&mut self, level: Level) {
        self.max_log_level = level;
    }

    pub fn set_save_path(&mut self, path: PathBuf) {
        self.save_path = path;
    }

    pub fn set_time_format(&mut self, format: String) {
        self.time_format = format;
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}