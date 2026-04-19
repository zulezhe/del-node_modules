use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use crate::logger::Logger;
use crate::i18n::I18n;
use crate::utils::should_ignore;

pub struct Scanner {
    target_path: PathBuf,
    ignore_dirs: Vec<String>,
    logger: Logger,
    i18n: I18n,
}

impl Scanner {
    pub fn new(target_path: PathBuf, ignore_dirs: Vec<String>, logger: Logger, i18n: I18n) -> Self {
        Self {
            target_path,
            ignore_dirs,
            logger,
            i18n,
        }
    }

    pub fn scan(&self) -> Vec<PathBuf> {
        self.logger.info(&self.i18n.t("scanning", &[("path", &self.target_path.to_string_lossy())]));

        let canonical_path = self.target_path.canonicalize().unwrap_or_else(|_| self.target_path.clone());
        let found_paths = Arc::new(Mutex::new(Vec::new()));
        let ignore_dirs = Arc::new(self.ignore_dirs.clone());
        let target_path = Arc::new(canonical_path.clone());

        // Collect top-level subdirectories for parallel scanning
        let subdirs: Vec<PathBuf> = fs::read_dir(&canonical_path)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| e.path())
                    .collect()
            })
            .unwrap_or_default();

        // Scan each top-level subdirectory in parallel
        subdirs.par_iter().for_each(|subdir| {
            let mut local_found = Vec::new();
            Self::scan_directory_recursive(
                subdir,
                &target_path,
                &ignore_dirs,
                &mut local_found,
            );
            if !local_found.is_empty() {
                let mut global = found_paths.lock().unwrap();
                global.extend(local_found);
            }
        });

        // Also check if the target path itself contains node_modules directly
        if let Ok(entries) = fs::read_dir(&canonical_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.file_name().map(|n| n == "node_modules").unwrap_or(false) {
                    found_paths.lock().unwrap().push(path);
                }
            }
        }

        let mut result = Arc::try_unwrap(found_paths).unwrap().into_inner().unwrap();
        result.sort();
        result
    }

    fn scan_directory_recursive(
        dir_path: &Path,
        target_path: &Path,
        ignore_dirs: &[String],
        found_paths: &mut Vec<PathBuf>,
    ) {
        if should_ignore(dir_path, target_path, ignore_dirs) {
            return;
        }

        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        // Pre-allocate buffer for better performance
        let mut subdirs = Vec::with_capacity(32);
        
        for entry in entries.flatten() {
            let full_path = entry.path();

            if should_ignore(&full_path, target_path, ignore_dirs) {
                continue;
            }

            // Use file_type() instead of metadata() for better performance
            match entry.file_type() {
                Ok(ft) => {
                    if ft.is_dir() {
                        if entry.file_name() == "node_modules" {
                            found_paths.push(full_path);
                        } else {
                            subdirs.push(full_path);
                        }
                    }
                }
                Err(_) => {
                    // Fallback to metadata() if file_type() fails
                    match entry.metadata() {
                        Ok(metadata) => {
                            if metadata.is_dir() {
                                if entry.file_name() == "node_modules" {
                                    found_paths.push(full_path);
                                } else {
                                    subdirs.push(full_path);
                                }
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
        }

        // Process subdirectories
        for subdir in subdirs {
            Self::scan_directory_recursive(&subdir, target_path, ignore_dirs, found_paths);
        }
    }
}
