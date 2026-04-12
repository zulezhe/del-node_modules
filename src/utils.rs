use std::fs;
use std::path::Path;

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 Bytes".to_string();
    }
    
    let k = 1024.0;
    let sizes = ["Bytes", "KB", "MB", "GB"];
    let i = (bytes as f64).log(k).floor() as usize;
    let i = i.min(sizes.len() - 1);
    
    let size = bytes as f64 / k.powi(i as i32);
    format!("{:.2} {}", size, sizes[i])
}

pub fn get_directory_size(dir_path: &Path) -> u64 {
    let mut size = 0;
    
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    size += get_directory_size(&path);
                } else {
                    size += metadata.len();
                }
            }
        }
    }
    
    size
}

pub fn should_ignore(dir_path: &Path, target_path: &Path, ignore_dirs: &[String]) -> bool {
    ignore_dirs.iter().any(|ignore_pattern| {
        let absolute_ignore = target_path.join(ignore_pattern);
        dir_path.starts_with(&absolute_ignore)
    })
}
