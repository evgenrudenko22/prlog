use std::error::Error;

pub struct LogEntry {
    pub level: String,
    pub time_stamp: String,
    pub target: String,
    pub message: String,
}

impl LogEntry {
    pub fn new(level: String, time_stamp: String, target: String, message: String) -> Self {
        Self {
            level,
            time_stamp,
            target,
            message,
        }
    }

    pub fn parse_line(line: &str) -> Result<LogEntry, Box<dyn Error>> {
        let parts: Vec<&str> = line.splitn(4, '|').collect();

        if parts.len() == 4 {
            let level = parts[0];
            let time_stamp = parts[1];
            let target = parts[2];
            let message = parts[3];

            return Ok(LogEntry::new(level.into(), time_stamp.into(), target.into(), message.into()))
        }
        Err("Must be only 4 elements".into())
    }
}