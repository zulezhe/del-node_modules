use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct I18n {
    language: String,
    translations: HashMap<String, String>,
    fallback_translations: HashMap<String, String>,
}

impl I18n {
    pub fn new(language: &str) -> Self {
        let translations = Self::load_translations(language);
        let fallback_translations = Self::load_translations("en-US");
        
        Self {
            language: language.to_string(),
            translations,
            fallback_translations,
        }
    }

    pub fn set_language(&mut self, language: &str) {
        if Self::is_language_supported(language) {
            self.language = language.to_string();
            self.translations = Self::load_translations(language);
        } else {
            eprintln!("Language {} not supported, using en-US", language);
            self.language = "en-US".to_string();
            self.translations = Self::load_translations("en-US");
        }
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn t(&self, key: &str, params: &[(&str, &str)]) -> String {
        let mut translation = self.translations.get(key)
            .or_else(|| self.fallback_translations.get(key))
            .unwrap_or(&key.to_string())
            .clone();

        for (param_key, param_value) in params {
            translation = translation.replace(&format!("{{{}}}", param_key), param_value);
        }

        translation
    }

    fn is_language_supported(language: &str) -> bool {
        matches!(language, "zh-CN" | "en-US")
    }

    fn load_translations(language: &str) -> HashMap<String, String> {
        let mut translations = HashMap::new();

        match language {
            "zh-CN" => {
                translations.insert("scanning".to_string(), "正在扫描 {path} 查找 node_modules 目录...".to_string());
                translations.insert("found".to_string(), "发现 node_modules: {path}".to_string());
                translations.insert("foundTotal".to_string(), "发现 {count} 个 node_modules 目录".to_string());
                translations.insert("noFound".to_string(), "未发现 node_modules 目录。".to_string());
                translations.insert("deleted".to_string(), "已删除: {path}".to_string());
                translations.insert("deletedWithSize".to_string(), "已删除: {path} ({size})".to_string());
                translations.insert("deletedFallback".to_string(), "已删除 (备用方法): {path}".to_string());
                translations.insert("deletedFallbackWithSize".to_string(), "已删除 (备用方法): {path} ({size})".to_string());
                translations.insert("errorDeleting".to_string(), "删除 {path} 时出错，尝试备用方法...".to_string());
                translations.insert("failedDelete".to_string(), "删除 {path} 失败: {error}".to_string());
                translations.insert("unableAccess".to_string(), "无法访问 {path}: {error}".to_string());
                translations.insert("unableRead".to_string(), "无法读取目录 {path}: {error}".to_string());
                translations.insert("unableGetSize".to_string(), "无法获取 {path} 的大小: {error}".to_string());
                translations.insert("notDirectory".to_string(), "{path} 不是一个目录".to_string());
                translations.insert("summaryTitle".to_string(), "摘要".to_string());
                translations.insert("totalDeleted".to_string(), "删除的目录总数: {count}".to_string());
                translations.insert("totalSpaceFreed".to_string(), "释放的空间总计: {size}".to_string());
                translations.insert("complete".to_string(), "完成！".to_string());
                translations.insert("deleting".to_string(), "正在删除 {name}".to_string());
                translations.insert("starting".to_string(), "开始...".to_string());
                translations.insert("ignored".to_string(), "已忽略: {path}".to_string());
                translations.insert("helpTitle".to_string(), "🗑️  dnm (Delete Node Modules) - 递归删除 node_modules 目录".to_string());
                translations.insert("helpUsage".to_string(), "用法:".to_string());
                translations.insert("helpUsageText".to_string(), "dnm [目录] [选项]".to_string());
                translations.insert("helpOptions".to_string(), "选项:".to_string());
                translations.insert("helpOptionHelp".to_string(), "显示此帮助信息".to_string());
                translations.insert("helpOptionInteractive".to_string(), "以交互模式运行".to_string());
                translations.insert("helpOptionSize".to_string(), "显示已删除目录的大小".to_string());
                translations.insert("helpOptionNoProgress".to_string(), "禁用进度条".to_string());
                translations.insert("helpOptionLogLevel".to_string(), "设置日志级别 (debug, info, warn, error)".to_string());
                translations.insert("helpOptionLogFile".to_string(), "将日志写入文件".to_string());
                translations.insert("helpOptionSilent".to_string(), "禁止控制台输出".to_string());
                translations.insert("helpOptionLanguage".to_string(), "设置语言 (zh-CN, en-US)".to_string());
                translations.insert("helpOptionIgnore".to_string(), "忽略指定目录（可多次使用）".to_string());
                translations.insert("helpOptionNoSafe".to_string(), "禁用安全模式（直接删除，不确认）".to_string());
                translations.insert("helpExamples".to_string(), "示例:".to_string());
                translations.insert("interactiveTitle".to_string(), "🗑️  交互模式 - dnm".to_string());
                translations.insert("promptTargetPath".to_string(), "输入要扫描的目录路径:".to_string());
                translations.insert("promptShowSize".to_string(), "计算并显示目录大小？".to_string());
                translations.insert("promptShowProgress".to_string(), "显示进度条？".to_string());
                translations.insert("promptLogLevel".to_string(), "选择日志级别:".to_string());
                translations.insert("promptUseLogFile".to_string(), "将日志保存到文件？".to_string());
                translations.insert("promptLogFile".to_string(), "输入日志文件路径:".to_string());
                translations.insert("promptProceed".to_string(), "⚠️  这将删除所有 node_modules 目录。是否继续？".to_string());
                translations.insert("promptIgnoreDirs".to_string(), "输入要忽略的目录（逗号分隔，留空表示不忽略）:".to_string());
                translations.insert("promptSafeMode".to_string(), "启用安全模式（删除前显示列表并确认）？".to_string());
                translations.insert("pathNotExist".to_string(), "路径不存在".to_string());
                translations.insert("pathNotDirectory".to_string(), "路径不是一个目录".to_string());
                translations.insert("operationCancelled".to_string(), "❌ 操作已取消。".to_string());
                translations.insert("targetDirectory".to_string(), "🔍 目标目录: ".to_string());
                translations.insert("cleanupSuccess".to_string(), "✅ 清理成功完成！".to_string());
                translations.insert("nothingToCleanup".to_string(), "ℹ️  没有需要清理的内容。".to_string());
                translations.insert("error".to_string(), "❌ 错误: {message}".to_string());
                translations.insert("cliTitle".to_string(), "🗑️  dnm - 清理工具".to_string());
                translations.insert("failedWriteLog".to_string(), "写入日志文件失败: {error}".to_string());
                translations.insert("logDebug".to_string(), "[调试]".to_string());
                translations.insert("logInfo".to_string(), "[信息]".to_string());
                translations.insert("logWarn".to_string(), "[警告]".to_string());
                translations.insert("logError".to_string(), "[错误]".to_string());
                translations.insert("logSuccess".to_string(), "[成功]".to_string());
                translations.insert("safeModeTitle".to_string(), "安全模式 - 发现的 node_modules 目录".to_string());
                translations.insert("safeModePrompt".to_string(), "输入要删除的序号（例如: 1-5 或 1,3,5），留空删除全部，输入 q 取消:".to_string());
                translations.insert("safeModeInvalidInput".to_string(), "无效输入，请重试".to_string());
                translations.insert("safeModeSelected".to_string(), "已选择删除 {count} 个目录".to_string());
                translations.insert("safeModeAll".to_string(), "已选择删除全部 {count} 个目录".to_string());
                translations.insert("safeModeCancelled".to_string(), "操作已取消".to_string());
                translations.insert("indexColumn".to_string(), "序号".to_string());
                translations.insert("pathColumn".to_string(), "路径".to_string());
                translations.insert("sizeColumn".to_string(), "大小".to_string());
                translations.insert("directoriesWord".to_string(), "个目录".to_string());
            }
            _ => {
                translations.insert("scanning".to_string(), "Scanning {path} for node_modules directories...".to_string());
                translations.insert("found".to_string(), "Found node_modules: {path}".to_string());
                translations.insert("foundTotal".to_string(), "Found {count} node_modules directories".to_string());
                translations.insert("noFound".to_string(), "No node_modules directories found.".to_string());
                translations.insert("deleted".to_string(), "Deleted: {path}".to_string());
                translations.insert("deletedWithSize".to_string(), "Deleted: {path} ({size})".to_string());
                translations.insert("deletedFallback".to_string(), "Deleted (fallback): {path}".to_string());
                translations.insert("deletedFallbackWithSize".to_string(), "Deleted (fallback): {path} ({size})".to_string());
                translations.insert("errorDeleting".to_string(), "Error deleting {path}, trying fallback method...".to_string());
                translations.insert("failedDelete".to_string(), "Failed to delete {path}: {error}".to_string());
                translations.insert("unableAccess".to_string(), "Unable to access {path}: {error}".to_string());
                translations.insert("unableRead".to_string(), "Unable to read directory {path}: {error}".to_string());
                translations.insert("unableGetSize".to_string(), "Unable to get size of {path}: {error}".to_string());
                translations.insert("notDirectory".to_string(), "{path} is not a directory".to_string());
                translations.insert("summaryTitle".to_string(), "SUMMARY".to_string());
                translations.insert("totalDeleted".to_string(), "Total directories deleted: {count}".to_string());
                translations.insert("totalSpaceFreed".to_string(), "Total space freed: {size}".to_string());
                translations.insert("complete".to_string(), "Complete!".to_string());
                translations.insert("deleting".to_string(), "Deleting {name}".to_string());
                translations.insert("starting".to_string(), "Starting...".to_string());
                translations.insert("ignored".to_string(), "Ignored: {path}".to_string());
                translations.insert("helpTitle".to_string(), "🗑️  dnm (Delete Node Modules) - Recursively delete node_modules directories".to_string());
                translations.insert("helpUsage".to_string(), "Usage:".to_string());
                translations.insert("helpUsageText".to_string(), "dnm [directory] [options]".to_string());
                translations.insert("helpOptions".to_string(), "Options:".to_string());
                translations.insert("helpOptionHelp".to_string(), "Show this help message".to_string());
                translations.insert("helpOptionInteractive".to_string(), "Run in interactive mode".to_string());
                translations.insert("helpOptionSize".to_string(), "Show size of deleted directories".to_string());
                translations.insert("helpOptionNoProgress".to_string(), "Disable progress bar".to_string());
                translations.insert("helpOptionLogLevel".to_string(), "Set log level (debug, info, warn, error)".to_string());
                translations.insert("helpOptionLogFile".to_string(), "Write logs to file".to_string());
                translations.insert("helpOptionSilent".to_string(), "Suppress console output".to_string());
                translations.insert("helpOptionLanguage".to_string(), "Set language (zh-CN, en-US)".to_string());
                translations.insert("helpOptionIgnore".to_string(), "Ignore specified directories (can be used multiple times)".to_string());
                translations.insert("helpOptionNoSafe".to_string(), "Disable safe mode (delete without confirmation)".to_string());
                translations.insert("helpExamples".to_string(), "Examples:".to_string());
                translations.insert("interactiveTitle".to_string(), "🗑️  Interactive Mode - dnm".to_string());
                translations.insert("promptTargetPath".to_string(), "Enter the directory path to scan:".to_string());
                translations.insert("promptShowSize".to_string(), "Calculate and show directory sizes?".to_string());
                translations.insert("promptShowProgress".to_string(), "Show progress bar?".to_string());
                translations.insert("promptLogLevel".to_string(), "Select log level:".to_string());
                translations.insert("promptUseLogFile".to_string(), "Save logs to a file?".to_string());
                translations.insert("promptLogFile".to_string(), "Enter log file path:".to_string());
                translations.insert("promptProceed".to_string(), "⚠️  This will delete all node_modules directories. Continue?".to_string());
                translations.insert("promptIgnoreDirs".to_string(), "Enter directories to ignore (comma-separated, leave empty for none):".to_string());
                translations.insert("promptSafeMode".to_string(), "Enable safe mode (show list and confirm before deleting)?".to_string());
                translations.insert("pathNotExist".to_string(), "Path does not exist".to_string());
                translations.insert("pathNotDirectory".to_string(), "Path is not a directory".to_string());
                translations.insert("operationCancelled".to_string(), "❌ Operation cancelled.".to_string());
                translations.insert("targetDirectory".to_string(), "🔍 Target directory: ".to_string());
                translations.insert("cleanupSuccess".to_string(), "✅ Cleanup completed successfully!".to_string());
                translations.insert("nothingToCleanup".to_string(), "ℹ️  Nothing to clean up.".to_string());
                translations.insert("error".to_string(), "❌ Error: {message}".to_string());
                translations.insert("cliTitle".to_string(), "🗑️  dnm - Cleanup Tool".to_string());
                translations.insert("failedWriteLog".to_string(), "Failed to write to log file: {error}".to_string());
                translations.insert("logDebug".to_string(), "[DEBUG]".to_string());
                translations.insert("logInfo".to_string(), "[INFO]".to_string());
                translations.insert("logWarn".to_string(), "[WARN]".to_string());
                translations.insert("logError".to_string(), "[ERROR]".to_string());
                translations.insert("logSuccess".to_string(), "[SUCCESS]".to_string());
                translations.insert("safeModeTitle".to_string(), "Safe Mode - Found node_modules Directories".to_string());
                translations.insert("safeModePrompt".to_string(), "Enter numbers to delete (e.g., 1-5 or 1,3,5), leave empty for all, or q to cancel:".to_string());
                translations.insert("safeModeInvalidInput".to_string(), "Invalid input, please try again".to_string());
                translations.insert("safeModeSelected".to_string(), "Selected {count} directories for deletion".to_string());
                translations.insert("safeModeAll".to_string(), "Selected all {count} directories for deletion".to_string());
                translations.insert("safeModeCancelled".to_string(), "Operation cancelled".to_string());
                translations.insert("indexColumn".to_string(), "Index".to_string());
                translations.insert("pathColumn".to_string(), "Path".to_string());
                translations.insert("sizeColumn".to_string(), "Size".to_string());
                translations.insert("directoriesWord".to_string(), "directories".to_string());
            }
        }

        translations
    }
}
