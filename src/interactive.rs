use std::io::{self, Write};
use std::path::PathBuf;
use colored::*;
use crate::i18n::I18n;

pub struct InteractiveOptions {
    pub target_path: PathBuf,
    pub show_size: bool,
    pub show_progress: bool,
    pub log_level: String,
    pub log_file: Option<String>,
    pub ignore_dirs: Vec<String>,
    pub safe_mode: bool,
    pub language: String,
}

fn detect_system_language() -> &'static str {
    // Check LANG env var (Unix/Linux/macOS/WSL)
    if let Ok(lang) = std::env::var("LANG") {
        if lang.to_lowercase().starts_with("zh") {
            return "zh-CN";
        }
    }

    // On Windows, use system UI language API
    #[cfg(target_os = "windows")]
    {
        unsafe {
            extern "system" {
                fn GetUserDefaultUILanguage() -> u16;
            }
            let lang_id = GetUserDefaultUILanguage();
            // Chinese: primary lang ID 0x04
            if (lang_id & 0xFF) == 0x04 {
                return "zh-CN";
            }
        }
    }

    "zh-CN"
}

pub fn run_interactive_mode(_i18n: &I18n) -> Option<InteractiveOptions> {
    let language = detect_system_language();
    let i18n = I18n::new(language);

    println!("\n{}", i18n.t("interactiveTitle", &[]).bold().cyan());

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

    Some(InteractiveOptions {
        target_path,
        show_size: false,
        show_progress: true,
        log_level: "info".to_string(),
        log_file: None,
        ignore_dirs: Vec::new(),
        safe_mode: true,
        language: language.to_string(),
    })
}

fn prompt_input(prompt: &str) -> Option<String> {
    print!("{} ", prompt);
    io::stdout().flush().ok()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_string())
}

pub fn show_safe_mode_list(found_paths: &[PathBuf], i18n: &I18n, _show_size: bool) -> Option<Vec<PathBuf>> {
    println!("\n{}", i18n.t("safeModeTitle", &[]).bold().cyan());
    println!("{}", "═══════════════════════════════════════════════════".cyan());
    println!();

    for (index, path) in found_paths.iter().enumerate() {
        let index_str = format!("[{}]", index + 1).yellow();
        let path_str = path.to_string_lossy().cyan();
        println!("{} {}", index_str, path_str);
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
