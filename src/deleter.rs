/*
 * @Author: 王超旭
 * @Date: 2026-04-19 22:12:01
 * @LastEditors: 王超旭
 * @LastEditTime: 2026-04-20 00:00:32
 * @Description: 
 */
use std::fs;
use std::path::{Path, PathBuf};
use crate::logger::Logger;
use crate::i18n::I18n;

pub struct Deleter {
    logger: Logger,
    i18n: I18n,
}

pub struct DeletionResult {
    pub deleted_paths: Vec<PathBuf>,
    pub total: usize,
}

impl Deleter {
    pub fn new(logger: Logger, i18n: I18n, _show_size: bool) -> Self {
        Self {
            logger,
            i18n,
        }
    }

    pub fn delete_directory(&self, dir_path: &Path) -> bool {
        // Only get parent name for debug logging
        let parent_name = dir_path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let status = self.i18n.t("deleting", &[("name", parent_name)]);
        self.logger.debug(&status);

        match fs::remove_dir_all(dir_path) {
            Ok(()) => {
                self.logger.success(&self.i18n.t("deleted", &[("path", &dir_path.to_string_lossy())]));
                true
            }
            Err(e) => {
                let error_msg = e.to_string();
                // Fast permission check using error kind first
                let is_permission_error = e.kind() == std::io::ErrorKind::PermissionDenied
                    || error_msg.contains("Permission denied")
                    || error_msg.contains("Access is denied")
                    || error_msg.contains("拒绝访问");

                if is_permission_error {
                    self.logger.error(&self.i18n.t("permissionDenied", &[
                        ("path", &dir_path.to_string_lossy()),
                    ]));
                } else {
                    self.logger.error(&self.i18n.t("failedDelete", &[
                        ("path", &dir_path.to_string_lossy()),
                        ("error", &error_msg),
                    ]));
                }
                false
            }
        }
    }
}
