use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use clap::Parser;
use colored::Colorize;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "prlog-cli", about = "Utility for easy reading logs from prlog", long_about = None)]
struct Args {
    path: String,

    #[arg(short, long)]
    target: Option<String>,
    #[arg(long)]
    json: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if !Path::new(&args.path).exists() {
        eprintln!("{} File doesn't exist: {}", "Error".red().bold(), args.path);
        return Ok(());
    }

    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let entry = prlog::entry::LogEntry::parse_line(line.as_str()).expect("Error while parsing log entry");

        if let Some(ref t) = args.target {
            if t != &entry.target { continue; }
        }

        if args.json {
            let json_entry = JsonLogEntry {
                level: entry.level,
                timestamp: entry.time_stamp,
                target: entry.target,
                message: entry.message,
            };
            entries.push(json_entry);
        } else {
            println!("{}", prlog::formatter::format_console(&entry.level, &entry.time_stamp, &entry.target, &entry.message));
        }
    }

    if args.json {
        let json_output = serde_json::to_string_pretty(&entries)?;
        println!("{}", json_output);
    }

    Ok(())
}

#[derive(Serialize)]
struct JsonLogEntry {
    level: String,
    timestamp: String,
    target: String,
    message: String,
}