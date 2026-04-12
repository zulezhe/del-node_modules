use std::io::{self, Write};
use std::path::PathBuf;
use colored::*;
use crate::i18n::I18n;
use crate::utils::{format_bytes, get_directory_size};

pub struct InteractiveOptions {
    pub target_path: PathBuf,
    pub show_size: bool,
    pub show_progress: bool,
    pub log_level: String,
    pub log_file: Option<String>,
    pub ignore_dirs: Vec<String>,
    pub safe_mode: bool,
}

pub fn run_interactive_mode(i18n: &I18n) -> Option<InteractiveOptions> {
    println!("\n{}", i18n.t("interactiveTitle", &[]).bold().cyan());

    println!("\n[1] 简体中文 (zh-CN)");
    println!("[2] English (en-US)");
    let lang_input = prompt_input("选择语言 / Select language: ")?;
    let language = match lang_input.trim() {
        "2" => "en-US",
        _ => "zh-CN",
    };
    let i18n = I18n::new(language);

    let default_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target_path = loop {
        let input = prompt_input(&i18n.t("promptTargetPath", &[]))
            .unwrap_or_else(|| default_path.to_string_lossy().to_string());
        let path = if input.is_empty() {
            default_path.clone()
        } else {
            PathBuf::from(&input)
        };

        if !path.exists() {
            println!("{}", i18n.t("pathNotExist", &[]).red());
            continue;
        }
        if !path.is_dir() {
            println!("{}", i18n.t("pathNotDirectory", &[]).red());
            continue;
        }
        break path.canonicalize().unwrap_or(path);
    };

    let ignore_input = prompt_input(&i18n.t("promptIgnoreDirs", &[])).unwrap_or_default();
    let ignore_dirs: Vec<String> = if ignore_input.trim().is_empty() {
        Vec::new()
    } else {
        ignore_input.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let safe_mode = prompt_confirm(&i18n.t("promptSafeMode", &[]), true);
    let show_size = prompt_confirm(&i18n.t("promptShowSize", &[]), false);
    let show_progress = prompt_confirm(&i18n.t("promptShowProgress", &[]), true);

    println!("\n日志级别 / Log level:");
    println!("[1] debug");
    println!("[2] info");
    println!("[3] warn");
    println!("[4] error");
    let log_level_input = prompt_input(&i18n.t("promptLogLevel", &[])).unwrap_or_else(|| "2".to_string());
    let log_level = match log_level_input.trim() {
        "1" => "debug",
        "2" => "info",
        "3" => "warn",
        "4" => "error",
        _ => "info",
    }.to_string();

    let use_log_file = prompt_confirm(&i18n.t("promptUseLogFile", &[]), false);
    let log_file = if use_log_file {
        let input = prompt_input(&i18n.t("promptLogFile", &[])).unwrap_or_else(|| "del-node-modules.log".to_string());
        Some(if input.is_empty() { "del-node-modules.log".to_string() } else { input })
    } else {
        None
    };

    println!("\n{}", format!("{} {}", i18n.t("targetDirectory", &[]), target_path.display()).cyan());
    println!("  Ignore: {}", if ignore_dirs.is_empty() { "None".to_string() } else { ignore_dirs.join(", ") }.cyan());
    println!("  Safe Mode: {}", if safe_mode { "Yes".to_string() } else { "No".to_string() }.cyan());
    println!("  Show Size: {}", if show_size { "Yes".to_string() } else { "No".to_string() }.cyan());
    println!("  Log Level: {}", log_level.cyan());
    if let Some(ref log) = log_file {
        println!("  Log File: {}", log.cyan());
    }
    println!();

    if !prompt_confirm(&i18n.t("promptProceed", &[]), false) {
        println!("\n{}", i18n.t("operationCancelled", &[]).red());
        return None;
    }

    Some(InteractiveOptions {
        target_path,
        show_size,
        show_progress,
        log_level,
        log_file,
        ignore_dirs,
        safe_mode,
    })
}

fn prompt_input(prompt: &str) -> Option<String> {
    print!("{} ", prompt);
    io::stdout().flush().ok()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_string())
}

fn prompt_confirm(prompt: &str, default: bool) -> bool {
    let default_str = if default { "Y/n" } else { "y/N" };
    let input = prompt_input(&format!("{} [{}]", prompt, default_str)).unwrap_or_default();
    
    if input.is_empty() {
        default
    } else {
        matches!(input.to_lowercase().as_str(), "y" | "yes")
    }
}

pub fn show_safe_mode_list(found_paths: &[PathBuf], i18n: &I18n, show_size: bool) -> Option<Vec<PathBuf>> {
    println!("\n{}", i18n.t("safeModeTitle", &[]).bold().cyan());
    println!("{}", "═══════════════════════════════════════════════════".cyan());
    println!();

    for (index, path) in found_paths.iter().enumerate() {
        let index_str = format!("[{}]", index + 1).yellow();
        let path_str = path.to_string_lossy().cyan();
        let size_str = if show_size {
            let size = get_directory_size(path);
            if size > 0 {
                format!(" ({})", format_bytes(size)).dimmed().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        println!("{} {}{}", index_str, path_str, size_str);
    }

    println!();
    let input = prompt_input(&i18n.t("safeModePrompt", &[]))?;
    let input = input.trim();

    if input.eq_ignore_ascii_case("q") {
        println!("{}", i18n.t("safeModeCancelled", &[]).yellow());
        return None;
    }

    if input.is_empty() {
        println!("{}", i18n.t("safeModeAll", &[("count", &found_paths.len().to_string())]).green());
        return Some(found_paths.to_vec());
    }

    match parse_selection(input, found_paths.len()) {
        Ok(indices) => {
            let selected_paths: Vec<PathBuf> = indices.iter()
                .map(|&i| found_paths[i - 1].clone())
                .collect();
            println!("{}", i18n.t("safeModeSelected", &[("count", &selected_paths.len().to_string())]).green());
            Some(selected_paths)
        }
        Err(_) => {
            println!("{}", i18n.t("safeModeInvalidInput", &[]).red());
            None
        }
    }
}

fn parse_selection(input: &str, max_index: usize) -> Result<Vec<usize>, String> {
    let mut indices = std::collections::HashSet::new();
    let parts: Vec<&str> = input.split(',').collect();

    for part in parts {
        let trimmed = part.trim();
        if trimmed.contains('-') {
            let range_parts: Vec<&str> = trimmed.split('-').collect();
            if range_parts.len() != 2 {
                return Err("Invalid range".to_string());
            }
            let start = range_parts[0].trim().parse::<usize>().map_err(|_| "Invalid number")?;
            let end = range_parts[1].trim().parse::<usize>().map_err(|_| "Invalid number")?;
            
            if start < 1 || end > max_index || start > end {
                return Err("Invalid range".to_string());
            }
            for i in start..=end {
                indices.insert(i);
            }
        } else {
            let num = trimmed.parse::<usize>().map_err(|_| "Invalid number")?;
            if num < 1 || num > max_index {
                return Err("Invalid number".to_string());
            }
            indices.insert(num);
        }
    }

    let mut sorted_indices: Vec<usize> = indices.into_iter().collect();
    sorted_indices.sort();
    Ok(sorted_indices)
}
