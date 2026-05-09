//! Module for working with the file system.
//!
//! Provides reliable logging to disk. Uses the DSV format
//! (Delimiter Separated Values) with a vertical bar `|` as a separator.
//! This avoids conflicts with colons in time and provides
//! high write speed and file compactness.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use crate::entry::LogEntry;

pub fn write(path_buf: PathBuf, entry: &LogEntry) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_buf.as_path())
    {
        let line = format!("{}|{}|{}|{}\n",
            entry.level, entry.time_stamp, entry.target, entry.message);

        let _ = file.write_all(line.as_bytes());
    }
}