use colored::*;
use std::io::Write;

pub fn info(message: &str) {
    println!("{} {}", "INFO:".blue().bold(), message);
    let _ = std::io::stdout().flush();
}

pub fn warn(message: &str) {
    eprintln!("{} {}", "WARN:".yellow().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{} {}", "ERROR:".red().bold(), message);
}
