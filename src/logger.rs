use std::fs;
use std::path::Path;
use chrono::Local;
use colored::*;
use crate::i18n::I18n;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Success,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }

    fn level_num(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Success => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
        }
    }
}

#[derive(Clone)]
pub struct Logger {
    log_level: LogLevel,
    log_file: Option<String>,
    silent: bool,
    i18n: I18n,
}

impl Logger {
    pub fn new(log_level: &str, log_file: Option<String>, silent: bool, i18n: I18n) -> Self {
        if let Some(ref path) = log_file {
            if let Some(parent) = Path::new(path).parent() {
                if !parent.exists() {
                    let _ = fs::create_dir_all(parent);
                }
            }
        }

        Self {
            log_level: LogLevel::from_str(log_level),
            log_file,
            silent,
            i18n,
        }
    }

    fn should_log(&self, level: &LogLevel) -> bool {
        level.level_num() >= self.log_level.level_num()
    }

    fn format_message(&self, level: &str, message: &str) -> String {
        let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
        format!("[{}] [{}] {}", timestamp, level.to_uppercase(), message)
    }

    fn write_to_file(&self, message: &str) {
        if let Some(ref path) = self.log_file {
            if let Err(e) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .and_then(|mut file| {
                    use std::io::Write;
                    writeln!(file, "{}", message)?;
                    Ok(())
                })
            {
                eprintln!("{}", self.i18n.t("failedWriteLog", &[("error", &e.to_string())]).red());
            }
        }
    }

    pub fn debug(&self, message: &str) {
        if !self.should_log(&LogLevel::Debug) {
            return;
        }
        let formatted = self.format_message("debug", message);
        if !self.silent {
            println!("{}", formatted.color(colored::Color::BrightBlack));
        }
        self.write_to_file(&formatted);
    }

    pub fn info(&self, message: &str) {
        if !self.should_log(&LogLevel::Info) {
            return;
        }
        let formatted = self.format_message("info", message);
        if !self.silent {
            println!("{}", formatted.blue());
        }
        self.write_to_file(&formatted);
    }

    pub fn warn(&self, message: &str) {
        if !self.should_log(&LogLevel::Warn) {
            return;
        }
        let formatted = self.format_message("warn", message);
        if !self.silent {
            println!("{}", formatted.yellow());
        }
        self.write_to_file(&formatted);
    }

    pub fn error(&self, message: &str) {
        if !self.should_log(&LogLevel::Error) {
            return;
        }
        let formatted = self.format_message("error", message);
        if !self.silent {
            println!("{}", formatted.red());
        }
        self.write_to_file(&formatted);
    }

    pub fn success(&self, message: &str) {
        if !self.should_log(&LogLevel::Success) {
            return;
        }
        let formatted = self.format_message("success", message);
        if !self.silent {
            println!("{}", formatted.green());
        }
        self.write_to_file(&formatted);
    }
}
