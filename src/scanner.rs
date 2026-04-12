use std::fs;
use std::path::{Path, PathBuf};
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
        
        let mut found_paths = Vec::new();
        let canonical_path = self.target_path.canonicalize().unwrap_or_else(|_| self.target_path.clone());
        self.scan_directory(&canonical_path, &mut found_paths);
        
        found_paths
    }

    fn scan_directory(&self, dir_path: &Path, found_paths: &mut Vec<PathBuf>) {
        if should_ignore(dir_path, &self.target_path, &self.ignore_dirs) {
            self.logger.debug(&self.i18n.t("ignored", &[("path", &dir_path.to_string_lossy())]));
            return;
        }

        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(e) => {
                self.logger.warn(&self.i18n.t("unableRead", &[
                    ("path", &dir_path.to_string_lossy()),
                    ("error", &e.to_string()),
                ]));
                return;
            }
        };

        for entry in entries.flatten() {
            let full_path = entry.path();

            if should_ignore(&full_path, &self.target_path, &self.ignore_dirs) {
                self.logger.debug(&self.i18n.t("ignored", &[("path", &full_path.to_string_lossy())]));
                continue;
            }

            match entry.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if entry.file_name() == "node_modules" {
                            self.logger.info(&self.i18n.t("found", &[("path", &full_path.to_string_lossy())]));
                            found_paths.push(full_path);
                        } else {
                            self.scan_directory(&full_path, found_paths);
                        }
                    }
                }
                Err(e) => {
                    self.logger.warn(&self.i18n.t("unableAccess", &[
                        ("path", &full_path.to_string_lossy()),
                        ("error", &e.to_string()),
                    ]));
                }
            }
        }
    }
}
