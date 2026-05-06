pub mod i18n;
pub mod logger;
pub mod scanner;
pub mod deleter;
pub mod utils;
pub mod interactive;
pub mod cli;
pub mod elevate;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use i18n::I18n;
use logger::Logger;
use scanner::Scanner;
use deleter::{Deleter, DeletionResult};
use interactive::show_safe_mode_list;
use indicatif::ProgressBar;
use colored::*;

/// On Windows, detect if the console was created for this process (i.e., double-clicked).
/// GetConsoleProcessList returns the number of processes attached to the console.
/// If it returns 1, only our process uses the console → double-clicked.
#[cfg(target_os = "windows")]
pub fn is_gui_console() -> bool {
    unsafe {
        #[link(name = "kernel32")]
        extern "system" {
            fn GetConsoleProcessList(lpdwProcessList: *mut u32, dwProcessCount: u32) -> u32;
        }
        let mut count = 0u32;
        GetConsoleProcessList(&mut count, 1) <= 1
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_gui_console() -> bool {
    false
}

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
        Some(Arc::new(pb))
    } else {
        None
    };

    let deleted_paths = Arc::new(Mutex::new(Vec::new()));
    let count = Arc::new(Mutex::new(0usize));

    // Parallel deletion using rayon
    paths_to_delete.par_iter().for_each(|dir_path| {
        let success = deleter.delete_directory(dir_path);
        if success {
            deleted_paths.lock().unwrap().push(dir_path.clone());
        }

        if let Some(ref pb) = progress_bar {
            let mut c = count.lock().unwrap();
            *c += 1;
            let parent_name = dir_path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            pb.set_position(*c as u64);
            pb.set_message(i18n.t("deleting", &[("name", parent_name)]));
        }
    });

    let deleted_paths = Arc::try_unwrap(deleted_paths).unwrap().into_inner().unwrap();

    if let Some(pb) = progress_bar {
        pb.finish_with_message(i18n.t("complete", &[]));
        println!();
    }

    if !options.silent {
        println!("\n{}", "═══════════════════════════════════════════════════".bold().green());
        println!("{}", format!("                    {}                         ", i18n.t("summaryTitle", &[])).bold().green());
        println!("{}", "═══════════════════════════════════════════════════".bold().green());
        println!("{}", i18n.t("totalDeleted", &[("count", &deleted_paths.len().to_string().yellow().to_string())]));
        println!("{}", "═══════════════════════════════════════════════════\n".bold().green());
    }

    Ok(DeletionResult {
        deleted_paths: deleted_paths.clone(),
        total: deleted_paths.len(),
    })
}

pub fn run_cli() -> i32 {
    use clap::Parser;
    use cli::CliArgs;
    use interactive::run_interactive_mode;

    let args = match CliArgs::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // Handle --help, --version, and parse errors without calling process::exit
            e.print().ok();
            return match e.use_stderr() {
                true => 1,
                false => 0,
            };
        }
    };
    let mut i18n = I18n::new(&args.lang);

    // Auto-enter interactive mode when double-clicked without arguments
    let mut args = args;
    if is_gui_console() && args.target_path.is_none() && !args.interactive {
        println!("{}", i18n.t("guiAutoInteractive", &[]).yellow());
        args.interactive = true;
    }

    // Handle elevation request
    if args.elevate {
        if elevate::is_elevated() {
            println!("{}", i18n.t("alreadyAdmin", &[]).green());
        } else {
            println!("{}", i18n.t("elevating", &[]).yellow());
            let all_args: Vec<String> = std::env::args().skip(1)
                .filter(|a| a != "--elevate")
                .collect();
            if let Err(e) = elevate::elevate(&all_args) {
                eprintln!("{}", i18n.t("elevateFailed", &[("error", &e)]).red());
                return 1;
            }
            // elevate() calls process::exit(0) on success, so we won't reach here
            unreachable!()
        }
    }

    if args.interactive {
        let mut continue_loop = true;
        while continue_loop {
            continue_loop = false;
            if let Some(opts) = run_interactive_mode(&i18n) {
                i18n = I18n::new(&opts.language);

                println!("\n{}", "╔════════════════════════════════════════════════╗".bold().magenta());
                println!("{}", "║     🗑️  dnm - 清理工具                        ║".bold().magenta());
                println!("{}", "╚════════════════════════════════════════════════╝\n".bold().magenta());

                let options = FindAndDeleteOptions {
                    show_progress: opts.show_progress,
                    show_size: false,
                    log_level: opts.log_level,
                    log_file: opts.log_file,
                    silent: false,
                    ignore: opts.ignore_dirs,
                    safe_mode: opts.safe_mode,
                    language: opts.language.clone(),
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
                    }
                }

                continue_loop = true;
            }
        }
        0
    } else {
        println!("\n{}", "╔════════════════════════════════════════════════╗".bold().magenta());
        println!("{}", "║     🗑️  dnm - 清理工具                        ║".bold().magenta());
        println!("{}", "╚════════════════════════════════════════════════╝\n".bold().magenta());

        let target_path = args.resolve_target_path();
        println!("{} {}\n", i18n.t("targetDirectory", &[]), target_path.display().to_string().white());

        let options = FindAndDeleteOptions {
            show_progress: !args.no_progress,
            show_size: false,
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
                0
            }
            Err(e) => {
                eprintln!("{}", i18n.t("error", &[("message", &e)]).bold().red());
                1
            }
        }
    }
}
