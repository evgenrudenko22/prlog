//! Module for styling and formatting output.
//!
//! Responsible for preparing lines for printing to the terminal.
//! If the `colors` feature is enabled, the module uses ANSI codes to colorize
//! different logging levels (Error - red, Warn - yellow, etc.),
//! which makes reading logs in the console much more convenient.

/// Function to format log information into pretty string
pub fn format_console(level: &str, time: &str, target: &str, msg: &str) -> String {
    #[cfg(feature = "colors")]
    {
        use colored::*;
        let level_colored = match level {
            "ERROR" => level.red().bold(),
            "WARN" => level.yellow(),
            "INFO" => level.green(),
            _ => level.normal(),
        };
        format!("[{}] [{}] [{}]: {}", level_colored, time.bright_black(), target.blue(), msg)
    }
    #[cfg(not(feature = "colors"))]
    {
        format!("[{}] [{}] [{}]: {}", level, time, target, msg)
    }
}