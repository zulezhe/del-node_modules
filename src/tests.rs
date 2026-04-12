#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::i18n::I18n;
    use crate::find_and_delete_node_modules;
    use crate::FindAndDeleteOptions;

    fn get_test_dir(name: &str) -> String {
        format!("test-temp-rust-{}-{}", name, std::process::id())
    }

    fn setup_test_structure(test_dir: &str) {
        cleanup(test_dir); // Ensure clean state

        fs::create_dir_all(format!("{}/project1/node_modules/package1", test_dir)).unwrap();
        fs::write(format!("{}/project1/node_modules/package1/index.js", test_dir), "// test file").unwrap();

        fs::create_dir_all(format!("{}/project2/node_modules/package2", test_dir)).unwrap();
        fs::create_dir_all(format!("{}/nested/sub/node_modules", test_dir)).unwrap();
        fs::create_dir_all(format!("{}/ignored/node_modules", test_dir)).unwrap();
    }

    fn cleanup(test_dir: &str) {
        let _ = fs::remove_dir_all(test_dir);
        if Path::new(test_dir).exists() {
            let _ = fs::remove_dir_all(test_dir);
        }
    }

    #[test]
    fn test_basic_deletion() {
        let test_dir = get_test_dir("basic");
        setup_test_structure(&test_dir);

        let options = FindAndDeleteOptions {
            safe_mode: false,
            silent: true,
            show_progress: false,
            ..Default::default()
        };

        let result = find_and_delete_node_modules(&test_dir, options);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.total, 4);
        
        assert!(!Path::new(&format!("{}/project1/node_modules", test_dir)).exists());
        assert!(!Path::new(&format!("{}/project2/node_modules", test_dir)).exists());
        assert!(!Path::new(&format!("{}/nested/sub/node_modules", test_dir)).exists());

        cleanup(&test_dir);
    }

    #[test]
    fn test_ignore_functionality() {
        let test_dir = get_test_dir("ignore");
        setup_test_structure(&test_dir);

        let options = FindAndDeleteOptions {
            safe_mode: false,
            silent: true,
            show_progress: false,
            ignore: vec!["ignored".to_string()],
            ..Default::default()
        };

        let result = find_and_delete_node_modules(&test_dir, options);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.total, 3);
        
        assert!(Path::new(&format!("{}/ignored/node_modules", test_dir)).exists());
        assert!(!Path::new(&format!("{}/project1/node_modules", test_dir)).exists());

        cleanup(&test_dir);
    }

    #[test]
    fn test_i18n_chinese() {
        let i18n = I18n::new("zh-CN");
        assert!(i18n.t("helpTitle", &[]).contains("dnm"));
        assert!(i18n.t("scanning", &[("path", "test")]).contains("正在扫描"));
    }

    #[test]
    fn test_i18n_english() {
        let i18n = I18n::new("en-US");
        assert!(i18n.t("helpTitle", &[]).contains("dnm"));
        assert!(i18n.t("scanning", &[("path", "test")]).contains("Scanning"));
    }

    #[test]
    fn test_empty_directory() {
        let test_dir = get_test_dir("empty");
        let empty_dir = format!("{}/empty", test_dir);
        fs::create_dir_all(&empty_dir).unwrap();

        let options = FindAndDeleteOptions {
            safe_mode: false,
            silent: true,
            show_progress: false,
            ..Default::default()
        };

        let result = find_and_delete_node_modules(&empty_dir, options);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.total, 0);

        cleanup(&test_dir);
    }

    #[test]
    fn test_multiple_ignore_patterns() {
        let test_dir = get_test_dir("multiple");
        setup_test_structure(&test_dir);

        fs::create_dir_all(format!("{}/backup/node_modules", test_dir)).unwrap();

        let options = FindAndDeleteOptions {
            safe_mode: false,
            silent: true,
            show_progress: false,
            ignore: vec!["ignored".to_string(), "backup".to_string()],
            ..Default::default()
        };

        let result = find_and_delete_node_modules(&test_dir, options);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.total, 3);
        
        assert!(Path::new(&format!("{}/ignored/node_modules", test_dir)).exists());
        assert!(Path::new(&format!("{}/backup/node_modules", test_dir)).exists());

        cleanup(&test_dir);
    }
}
