const translations = {
  'zh-CN': {
    scanning: '正在扫描 {path} 查找 node_modules 目录...',
    found: '发现 node_modules: {path}',
    foundTotal: '发现 {count} 个 node_modules 目录',
    noFound: '未发现 node_modules 目录。',
    deleted: '已删除: {path}',
    deletedWithSize: '已删除: {path} ({size})',
    deletedFallback: '已删除 (备用方法): {path}',
    deletedFallbackWithSize: '已删除 (备用方法): {path} ({size})',
    errorDeleting: '删除 {path} 时出错，尝试备用方法...',
    failedDelete: '删除 {path} 失败: {error}',
    unableAccess: '无法访问 {path}: {error}',
    unableRead: '无法读取目录 {path}: {error}',
    unableGetSize: '无法获取 {path} 的大小: {error}',
    notDirectory: '{path} 不是一个目录',
    summary: '摘要',
    summaryTitle: '摘要',
    totalDeleted: '删除的目录总数: {count}',
    totalSpaceFreed: '释放的空间总计: {size}',
    complete: '完成！',
    deleting: '正在删除 {name}',
    starting: '开始...',
    ignored: '已忽略: {path}',
    
    helpTitle: '🗑️  dnm (Delete Node Modules) - 递归删除 node_modules 目录',
    helpUsage: '用法:',
    helpUsageText: 'dnm [目录] [选项]',
    helpOptions: '选项:',
    helpOptionHelp: '显示此帮助信息',
    helpOptionInteractive: '以交互模式运行',
    helpOptionSize: '显示已删除目录的大小',
    helpOptionNoProgress: '禁用进度条',
    helpOptionLogLevel: '设置日志级别 (debug, info, warn, error)',
    helpOptionLogFile: '将日志写入文件',
    helpOptionSilent: '禁止控制台输出',
    helpOptionLanguage: '设置语言 (zh-CN, en-US)',
    helpOptionIgnore: '忽略指定目录（可多次使用）',
    helpOptionNoSafe: '禁用安全模式（直接删除，不确认）',
    helpExamples: '示例:',
    
    interactiveTitle: '🗑️  交互模式 - dnm',
    promptTargetPath: '输入要扫描的目录路径:',
    promptShowSize: '计算并显示目录大小？',
    promptShowProgress: '显示进度条？',
    promptLogLevel: '选择日志级别:',
    promptUseLogFile: '将日志保存到文件？',
    promptLogFile: '输入日志文件路径:',
    promptProceed: '⚠️  这将删除所有 node_modules 目录。是否继续？',
    promptIgnoreDirs: '输入要忽略的目录（逗号分隔，留空表示不忽略）:',
    promptSafeMode: '启用安全模式（删除前显示列表并确认）？',
    
    pathNotExist: '路径不存在',
    pathNotDirectory: '路径不是一个目录',
    operationCancelled: '❌ 操作已取消。',
    
    targetDirectory: '🔍 目标目录: ',
    cleanupSuccess: '✅ 清理成功完成！',
    nothingToCleanup: 'ℹ️  没有需要清理的内容。',
    error: '❌ 错误: {message}',
    
    cliTitle: '🗑️  dnm - 清理工具',
    
    failedWriteLog: '写入日志文件失败: {error}',
    
    logDebug: '[调试]',
    logInfo: '[信息]',
    logWarn: '[警告]',
    logError: '[错误]',
    logSuccess: '[成功]',
    
    safeModeTitle: '安全模式 - 发现的 node_modules 目录',
    safeModePrompt: '输入要删除的序号（例如: 1-5 或 1,3,5），留空删除全部，输入 q 取消:',
    safeModeInvalidInput: '无效输入，请重试',
    safeModeSelected: '已选择删除 {count} 个目录',
    safeModeAll: '已选择删除全部 {count} 个目录',
    safeModeCancelled: '操作已取消',
    
    indexColumn: '序号',
    pathColumn: '路径',
    sizeColumn: '大小'
  },
  
  'en-US': {
    scanning: 'Scanning {path} for node_modules directories...',
    found: 'Found node_modules: {path}',
    foundTotal: 'Found {count} node_modules directories',
    noFound: 'No node_modules directories found.',
    deleted: 'Deleted: {path}',
    deletedWithSize: 'Deleted: {path} ({size})',
    deletedFallback: 'Deleted (fallback): {path}',
    deletedFallbackWithSize: 'Deleted (fallback): {path} ({size})',
    errorDeleting: 'Error deleting {path}, trying fallback method...',
    failedDelete: 'Failed to delete {path}: {error}',
    unableAccess: 'Unable to access {path}: {error}',
    unableRead: 'Unable to read directory {path}: {error}',
    unableGetSize: 'Unable to get size of {path}: {error}',
    notDirectory: '{path} is not a directory',
    summary: 'Summary',
    summaryTitle: 'SUMMARY',
    totalDeleted: 'Total directories deleted: {count}',
    totalSpaceFreed: 'Total space freed: {size}',
    complete: 'Complete!',
    deleting: 'Deleting {name}',
    starting: 'Starting...',
    ignored: 'Ignored: {path}',
    
    helpTitle: '🗑️  dnm (Delete Node Modules) - Recursively delete node_modules directories',
    helpUsage: 'Usage:',
    helpUsageText: 'dnm [directory] [options]',
    helpOptions: 'Options:',
    helpOptionHelp: 'Show this help message',
    helpOptionInteractive: 'Run in interactive mode',
    helpOptionSize: 'Show size of deleted directories',
    helpOptionNoProgress: 'Disable progress bar',
    helpOptionLogLevel: 'Set log level (debug, info, warn, error)',
    helpOptionLogFile: 'Write logs to file',
    helpOptionSilent: 'Suppress console output',
    helpOptionLanguage: 'Set language (zh-CN, en-US)',
    helpOptionIgnore: 'Ignore specified directories (can be used multiple times)',
    helpOptionNoSafe: 'Disable safe mode (delete without confirmation)',
    helpExamples: 'Examples:',
    
    interactiveTitle: '🗑️  Interactive Mode - dnm',
    promptTargetPath: 'Enter the directory path to scan:',
    promptShowSize: 'Calculate and show directory sizes?',
    promptShowProgress: 'Show progress bar?',
    promptLogLevel: 'Select log level:',
    promptUseLogFile: 'Save logs to a file?',
    promptLogFile: 'Enter log file path:',
    promptProceed: '⚠️  This will delete all node_modules directories. Continue?',
    promptIgnoreDirs: 'Enter directories to ignore (comma-separated, leave empty for none):',
    promptSafeMode: 'Enable safe mode (show list and confirm before deleting)?',
    
    pathNotExist: 'Path does not exist',
    pathNotDirectory: 'Path is not a directory',
    operationCancelled: '❌ Operation cancelled.',
    
    targetDirectory: '🔍 Target directory: ',
    cleanupSuccess: '✅ Cleanup completed successfully!',
    nothingToCleanup: 'ℹ️  Nothing to clean up.',
    error: '❌ Error: {message}',
    
    cliTitle: '🗑️  dnm - Cleanup Tool',
    
    failedWriteLog: 'Failed to write to log file: {error}',
    
    logDebug: '[DEBUG]',
    logInfo: '[INFO]',
    logWarn: '[WARN]',
    logError: '[ERROR]',
    logSuccess: '[SUCCESS]',
    
    safeModeTitle: 'Safe Mode - Found node_modules Directories',
    safeModePrompt: 'Enter numbers to delete (e.g., 1-5 or 1,3,5), leave empty for all, or q to cancel:',
    safeModeInvalidInput: 'Invalid input, please try again',
    safeModeSelected: 'Selected {count} directories for deletion',
    safeModeAll: 'Selected all {count} directories for deletion',
    safeModeCancelled: 'Operation cancelled',
    
    indexColumn: 'Index',
    pathColumn: 'Path',
    sizeColumn: 'Size'
  }
};

class I18n {
  constructor(language = 'zh-CN') {
    this.language = language;
    this.fallbackLanguage = 'en-US';
  }
  
  setLanguage(language) {
    if (translations[language]) {
      this.language = language;
    } else {
      console.warn(`Language ${language} not supported, using ${this.fallbackLanguage}`);
      this.language = this.fallbackLanguage;
    }
  }
  
  t(key, params = {}) {
    let translation = translations[this.language]?.[key] || 
                     translations[this.fallbackLanguage]?.[key] || 
                     key;
    
    Object.keys(params).forEach(param => {
      translation = translation.replace(`{${param}}`, params[param]);
    });
    
    return translation;
  }
  
  getAvailableLanguages() {
    return Object.keys(translations);
  }
}

module.exports = I18n;
