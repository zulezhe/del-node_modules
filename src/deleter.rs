use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
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

        // 尝试多种删除策略，针对 node_modules 优化
        let success = if cfg!(windows) {
            // Windows: 优先使用 rd /s /q，对大量小文件最优
            self.delete_windows_cmd(dir_path)
                || self.delete_windows_powershell(dir_path)
                || self.delete_fallback_windows(dir_path)
        } else {
            // Unix: 优先使用 rm -rf
            self.delete_unix_cmd(dir_path)
                || fs::remove_dir_all(dir_path).is_ok()
        };

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
            self.logger.error(&self.i18n.t("failedDelete", &[
                ("path", &dir_path.to_string_lossy()),
                ("error", &"All deletion methods failed".to_string()),
            ]));
            (false, size)
        }
    }

    /// Windows: 使用 rd /s /q 命令 (对大量小文件最快)
    fn delete_windows_cmd(&self, dir_path: &Path) -> bool {
        for retry in 0..3 {
            let result = Command::new("cmd")
                .arg("/C")
                .arg(format!("rd /s /q \"{}\"", dir_path.display()))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            
            if let Ok(status) = result {
                if status.success() {
                    return true;
                }
            }
            
            // 文件被占用时重试，递增等待时间
            if retry < 2 {
                thread::sleep(Duration::from_millis(100 * (retry + 1) as u64));
            }
        }
        false
    }

    /// Windows: PowerShell Remove-Item (备选方案)
    fn delete_windows_powershell(&self, dir_path: &Path) -> bool {
        let result = Command::new("powershell")
            .arg("-Command")
            .arg(format!("Remove-Item -LiteralPath \"{}\" -Force -Recurse -ErrorAction SilentlyContinue", 
                        dir_path.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        
        result.map_or(false, |s| s.success())
    }

    /// Unix: 使用 rm -rf 命令
    fn delete_unix_cmd(&self, dir_path: &Path) -> bool {
        let result = Command::new("sh")
            .arg("-c")
            .arg(format!("rm -rf \"{}\"", dir_path.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        
        result.map_or(false, |s| s.success())
    }

    /// Windows: fs::remove_dir_all 作为最后备选项 (带重试)
    fn delete_fallback_windows(&self, dir_path: &Path) -> bool {
        for retry in 0..5 {
            match fs::remove_dir_all(dir_path) {
                Ok(_) => return true,
                Err(_) => {
                    if retry < 4 {
                        // 递增等待：200ms, 400ms, 600ms, 800ms
                        thread::sleep(Duration::from_millis(200 * (retry + 1) as u64));
                    }
                }
            }
        }
        false
    }
}
