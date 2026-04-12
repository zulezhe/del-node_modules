use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::logger::Logger;
use crate::i18n::I18n;
use crate::utils::{format_bytes, get_directory_size};

pub struct Deleter {
    logger: Logger,
    i18n: I18n,
    show_size: bool,
}

pub struct DeletionResult {
    pub deleted_paths: Vec<PathBuf>,
    pub total: usize,
    pub total_size: u64,
}

impl Deleter {
    pub fn new(logger: Logger, i18n: I18n, show_size: bool) -> Self {
        Self {
            logger,
            i18n,
            show_size,
        }
    }

    pub fn delete_directory(&self, dir_path: &Path) -> (bool, u64) {
        let size = if self.show_size {
            get_directory_size(dir_path)
        } else {
            0
        };

        let parent_name = dir_path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let status = self.i18n.t("deleting", &[("name", parent_name)]);
        self.logger.debug(&status);

        let success = self.delete_native(dir_path);

        if success {
            if self.show_size && size > 0 {
                let size_str = format_bytes(size);
                self.logger.success(&self.i18n.t("deletedWithSize", &[
                    ("path", &dir_path.to_string_lossy()),
                    ("size", &size_str),
                ]));
            } else {
                self.logger.success(&self.i18n.t("deleted", &[("path", &dir_path.to_string_lossy())]));
            }
            (true, size)
        } else {
            self.logger.warn(&self.i18n.t("errorDeleting", &[("path", &dir_path.to_string_lossy())]));
            
            let fallback_success = self.delete_fallback(dir_path);
            if fallback_success {
                if self.show_size && size > 0 {
                    let size_str = format_bytes(size);
                    self.logger.success(&self.i18n.t("deletedFallbackWithSize", &[
                        ("path", &dir_path.to_string_lossy()),
                        ("size", &size_str),
                    ]));
                } else {
                    self.logger.success(&self.i18n.t("deletedFallback", &[("path", &dir_path.to_string_lossy())]));
                }
                (true, size)
            } else {
                self.logger.error(&self.i18n.t("failedDelete", &[
                    ("path", &dir_path.to_string_lossy()),
                    ("error", &"All deletion methods failed".to_string()),
                ]));
                (false, 0)
            }
        }
    }

    fn delete_native(&self, dir_path: &Path) -> bool {
        if cfg!(windows) {
            let output = Command::new("cmd")
                .args(["/C", &format!("rd /s /q \"{}\"", dir_path.display())])
                .output();
            
            match output {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        } else {
            let output = Command::new("sh")
                .args(["-c", &format!("rm -rf \"{}\"", dir_path.display())])
                .output();
            
            match output {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }

    fn delete_fallback(&self, dir_path: &Path) -> bool {
        fs::remove_dir_all(dir_path).is_ok()
    }
}
