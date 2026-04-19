/*
 * @Author: 王超旭
 * @Date: 2026-04-19 20:19:38
 * @LastEditors: 王超旭
 * @LastEditTime: 2026-04-19 23:53:29
 * @Description: 
 */
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 Bytes".to_string();
    }
    
    const K: f64 = 1024.0;
    const SIZES: &[&str] = &["Bytes", "KB", "MB", "GB"];
    let i = (bytes as f64).log(K).floor() as usize;
    let i = i.min(SIZES.len() - 1);
    
    let size = bytes as f64 / K.powi(i as i32);
    format!("{:.2} {}", size, SIZES[i])
}

pub fn get_directory_size(dir_path: &Path) -> u64 {
    let total_size = Arc::new(AtomicU64::new(0));
    
    // Collect entries first
    let entries: Vec<_> = fs::read_dir(dir_path)
        .map(|e| e.filter_map(|entry| entry.ok()).collect())
        .unwrap_or_default();
    
    // Process entries in parallel using rayon
    use rayon::prelude::*;
    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        
        // Use file_type() for better performance
        match entry.file_type() {
            Ok(ft) => {
                if ft.is_dir() {
                    total_size.fetch_add(get_directory_size(&path), Ordering::Relaxed);
                } else if ft.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        total_size.fetch_add(metadata.len(), Ordering::Relaxed);
                    }
                }
            }
            Err(_) => {
                // Fallback to metadata() if file_type() fails
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        total_size.fetch_add(get_directory_size(&path), Ordering::Relaxed);
                    } else if metadata.is_file() {
                        total_size.fetch_add(metadata.len(), Ordering::Relaxed);
                    }
                }
            }
        }
    });
    
    total_size.load(Ordering::Relaxed)
}

pub fn should_ignore(dir_path: &Path, target_path: &Path, ignore_dirs: &[String]) -> bool {
    ignore_dirs.iter().any(|ignore_pattern| {
        let absolute_ignore = target_path.join(ignore_pattern);
        dir_path.starts_with(&absolute_ignore)
    })
}
