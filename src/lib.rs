pub mod i18n;
pub mod logger;
pub mod scanner;
pub mod deleter;
pub mod utils;
pub mod interactive;
pub mod cli;

#[cfg(test)]
mod tests;

use std::path::Path;
use i18n::I18n;
use logger::Logger;
use scanner::Scanner;
use deleter::{Deleter, DeletionResult};
use interactive::show_safe_mode_list;
use indicatif::ProgressBar;
use colored::*;

pub struct FindAndDeleteOptions {
    pub show_progress: bool,
    pub show_size: bool,
    pub log_level: String,
    pub log_file: Option<String>,
    pub silent: bool,
    pub ignore: Vec<String>,
    pub safe_mode: bool,
    pub language: String,
}

impl Default for FindAndDeleteOptions {
    fn default() -> Self {
        Self {
            show_progress: true,
            show_size: false,
            log_level: "info".to_string(),
            log_file: None,
            silent: false,
            ignore: Vec::new(),
            safe_mode: true,
            language: "zh-CN".to_string(),
        }
    }
}

pub fn find_and_delete_node_modules<P: AsRef<Path>>(target_path: P, options: FindAndDeleteOptions) -> Result<DeletionResult, String> {
    let target_path = target_path.as_ref().to_path_buf();
    let target_path = target_path.canonicalize().unwrap_or(target_path);
    let i18n = I18n::new(&options.language);
    
    let logger = Logger::new(
        &options.log_level,
        options.log_file.clone(),
        options.silent,
        i18n.clone(),
    );

    if !target_path.exists() {
        return Err(format!("Path does not exist: {}", target_path.display()));
    }

    if !target_path.is_dir() {
        return Err(i18n.t("notDirectory", &[("path", &target_path.to_string_lossy())]));
    }

    let scanner = Scanner::new(
        target_path.clone(),
        options.ignore.clone(),
        logger.clone(),
        i18n.clone(),
    );

    let found_paths = scanner.scan();

    if found_paths.is_empty() {
        if !options.silent {
            println!("\n{}", i18n.t("noFound", &[]).yellow());
        }
        return Ok(DeletionResult {
            deleted_paths: Vec::new(),
            total: 0,
            total_size: 0,
        });
    }

    if !options.silent {
        println!("{}", i18n.t("foundTotal", &[("count", &found_paths.len().to_string().yellow().to_string())]));
    }

    let mut paths_to_delete = found_paths.clone();

    if options.safe_mode && !options.silent {
        if let Some(selected) = show_safe_mode_list(&found_paths, &i18n, options.show_size) {
            paths_to_delete = selected;
        } else {
            return Ok(DeletionResult {
                deleted_paths: Vec::new(),
                total: 0,
                total_size: 0,
            });
        }
    }

    let deleter = Deleter::new(
        logger.clone(),
        i18n.clone(),
        options.show_size,
    );

    let progress_bar = if options.show_progress && !options.silent {
        let pb = ProgressBar::new(paths_to_delete.len() as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{bar} | {percent}% | {pos}/{len} directories | {status}")
                .unwrap()
                .progress_chars("█░"),
        );
        pb.set_message(i18n.t("starting", &[]));
        Some(pb)
    } else {
        None
    };

    let mut deleted_paths = Vec::new();
    let mut total_size: u64 = 0;

    for (index, dir_path) in paths_to_delete.iter().enumerate() {
        if let Some(ref pb) = progress_bar {
            pb.set_position(index as u64);
            let parent_name = dir_path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            pb.set_message(i18n.t("deleting", &[("name", parent_name)]));
        }

        let (success, size) = deleter.delete_directory(dir_path);
        if success {
            deleted_paths.push(dir_path.clone());
            total_size += size;
        }

        if let Some(ref pb) = progress_bar {
            pb.set_position((index + 1) as u64);
        }
    }

    if let Some(ref pb) = progress_bar {
        pb.finish_with_message(i18n.t("complete", &[]));
        println!();
    }

    if !options.silent {
        println!("\n{}", "═══════════════════════════════════════════════════".bold().green());
        println!("{}", format!("                    {}                         ", i18n.t("summaryTitle", &[])).bold().green());
        println!("{}", "═══════════════════════════════════════════════════".bold().green());
        println!("{}", i18n.t("totalDeleted", &[("count", &deleted_paths.len().to_string().yellow().to_string())]));
        if options.show_size && total_size > 0 {
            println!("{}", i18n.t("totalSpaceFreed", &[("size", &utils::format_bytes(total_size).yellow().to_string())]));
        }
        println!("{}", "═══════════════════════════════════════════════════\n".bold().green());
    }

    Ok(DeletionResult {
        deleted_paths: deleted_paths.clone(),
        total: deleted_paths.len(),
        total_size,
    })
}

pub fn run_cli() {
    use clap::Parser;
    use cli::CliArgs;
    use interactive::run_interactive_mode;

    let args = CliArgs::parse();
    let mut i18n = I18n::new(&args.lang);

    if args.interactive {
        if let Some(opts) = run_interactive_mode(&i18n) {
            i18n = I18n::new("zh-CN");
            
            println!("\n{}", "╔════════════════════════════════════════════════╗".bold().magenta());
            println!("{}", "║     🗑️  dnm - 清理工具                        ║".bold().magenta());
            println!("{}", "╚════════════════════════════════════════════════╝\n".bold().magenta());

            let options = FindAndDeleteOptions {
                show_progress: opts.show_progress,
                show_size: opts.show_size,
                log_level: opts.log_level,
                log_file: opts.log_file,
                silent: false,
                ignore: opts.ignore_dirs,
                safe_mode: opts.safe_mode,
                language: "zh-CN".to_string(),
            };

            match find_and_delete_node_modules(&opts.target_path, options) {
                Ok(result) => {
                    if result.total > 0 {
                        println!("{}", i18n.t("cleanupSuccess", &[]).bold().green());
                    } else {
                        println!("{}", i18n.t("nothingToCleanup", &[]).yellow());
                    }
                }
                Err(e) => {
                    eprintln!("{}", i18n.t("error", &[("message", &e)]).bold().red());
                    std::process::exit(1);
                }
            }
        } else {
            std::process::exit(0);
        }
    } else {
        println!("\n{}", "╔════════════════════════════════════════════════╗".bold().magenta());
        println!("{}", "║     🗑️  dnm - 清理工具                        ║".bold().magenta());
        println!("{}", "╚════════════════════════════════════════════════╝\n".bold().magenta());

        let target_path = args.resolve_target_path();
        println!("{} {}\n", i18n.t("targetDirectory", &[]), target_path.display().to_string().white());

        let options = FindAndDeleteOptions {
            show_progress: !args.no_progress,
            show_size: args.size,
            log_level: args.log_level.clone(),
            log_file: args.log_file.as_ref().map(|p| p.to_string_lossy().to_string()),
            silent: args.silent,
            ignore: args.ignore.clone(),
            safe_mode: !args.no_safe,
            language: args.lang.clone(),
        };

        i18n = I18n::new(&args.lang);

        match find_and_delete_node_modules(&target_path, options) {
            Ok(result) => {
                if result.total > 0 {
                    println!("{}", i18n.t("cleanupSuccess", &[]).bold().green());
                } else {
                    println!("{}", i18n.t("nothingToCleanup", &[]).yellow());
                }
            }
            Err(e) => {
                eprintln!("{}", i18n.t("error", &[("message", &e)]).bold().red());
                std::process::exit(1);
            }
        }
    }
}
