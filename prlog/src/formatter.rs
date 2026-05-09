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
        format!("[{}] [{}] [{}]: {}", level_colored, time.normal(), target.blue(), msg)
    }
    #[cfg(not(feature = "colors"))]
    {
        format!("[{}] [{}] [{}]: {}", level, time, target, msg)
    }
}