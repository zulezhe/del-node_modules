const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const chalk = require('chalk');
const cliProgress = require('cli-progress');
const Logger = require('./lib/logger');
const I18n = require('./lib/i18n');

async function findAndDeleteNodeModules(targetPath, options = {}) {
  const i18n = options.i18n || new I18n('zh-CN');
  
  const logger = new Logger({
    logLevel: options.logLevel || 'info',
    logFile: options.logFile,
    silent: options.silent || false,
    i18n: i18n
  });
  
  const stats = fs.statSync(targetPath);
  
  if (!stats.isDirectory()) {
    logger.error(i18n.t('notDirectory', { path: targetPath }));
    return;
  }

  const deletedPaths = [];
  const foundPaths = [];
  const ignoreDirs = options.ignore || [];
  const safeMode = options.safeMode !== false;
  let totalSize = 0;
  let progressBar = null;
  
  if (options.showProgress && !options.silent) {
    progressBar = new cliProgress.SingleBar({
      format: chalk.cyan('{bar}') + ' | {percentage}% | {value}/{total} ' + (i18n.language === 'zh-CN' ? '个目录' : 'directories') + ' | {status}',
      barCompleteChar: '\u2588',
      barIncompleteChar: '\u2591',
      hideCursor: true
    });
  }
  
  function getDirectorySize(dirPath) {
    let size = 0;
    try {
      const items = fs.readdirSync(dirPath);
      for (const item of items) {
        const fullPath = path.join(dirPath, item);
        try {
          const itemStats = fs.statSync(fullPath);
          if (itemStats.isDirectory()) {
            size += getDirectorySize(fullPath);
          } else {
            size += itemStats.size;
          }
        } catch (err) {
          logger.debug(i18n.t('unableGetSize', { path: fullPath, error: err.message }));
        }
      }
    } catch (err) {
      logger.debug(i18n.t('unableRead', { path: dirPath, error: err.message }));
    }
    return size;
  }
  
  function shouldIgnore(dirPath) {
    return ignoreDirs.some(ignorePattern => {
      const absoluteIgnore = path.resolve(targetPath, ignorePattern);
      return dirPath.startsWith(absoluteIgnore);
    });
  }
  
  function scanDirectory(dirPath) {
    if (shouldIgnore(dirPath)) {
      logger.debug(i18n.t('ignored', { path: chalk.gray(dirPath) }));
      return;
    }
    
    try {
      const items = fs.readdirSync(dirPath);
      
      for (const item of items) {
        const fullPath = path.join(dirPath, item);
        
        if (shouldIgnore(fullPath)) {
          logger.debug(i18n.t('ignored', { path: chalk.gray(fullPath) }));
          continue;
        }
        
        try {
          const itemStats = fs.statSync(fullPath);
          
          if (itemStats.isDirectory()) {
            if (item === 'node_modules') {
              logger.info(i18n.t('found', { path: chalk.cyan(fullPath) }));
              foundPaths.push(fullPath);
            } else {
              scanDirectory(fullPath);
            }
          }
        } catch (err) {
          logger.warn(i18n.t('unableAccess', { path: fullPath, error: err.message }));
        }
      }
    } catch (err) {
      logger.warn(i18n.t('unableRead', { path: dirPath, error: err.message }));
    }
  }
  
  function deleteDirectory(dirPath, index, total) {
    const status = i18n.t('deleting', { name: path.basename(path.dirname(dirPath)) });
    if (progressBar) {
      progressBar.update(index, { status });
    }
    
    let size = 0;
    if (options.showSize) {
      size = getDirectorySize(dirPath);
    }
    
    try {
      if (process.platform === 'win32') {
        execSync(`rd /s /q "${dirPath}"`, { stdio: 'ignore' });
      } else {
        execSync(`rm -rf "${dirPath}"`, { stdio: 'ignore' });
      }
      
      deletedPaths.push(dirPath);
      totalSize += size;
      
      if (options.showSize && size > 0) {
        const sizeStr = formatBytes(size);
        logger.success(i18n.t('deletedWithSize', { path: chalk.cyan(dirPath), size: chalk.gray(sizeStr) }));
      } else {
        logger.success(i18n.t('deleted', { path: chalk.cyan(dirPath) }));
      }
    } catch (err) {
      logger.warn(i18n.t('errorDeleting', { path: dirPath }));
      try {
        fs.rmSync(dirPath, { recursive: true, force: true });
        deletedPaths.push(dirPath);
        totalSize += size;
        
        if (options.showSize && size > 0) {
          const sizeStr = formatBytes(size);
          logger.success(i18n.t('deletedFallbackWithSize', { path: chalk.cyan(dirPath), size: chalk.gray(sizeStr) }));
        } else {
          logger.success(i18n.t('deletedFallback', { path: chalk.cyan(dirPath) }));
        }
      } catch (fallbackErr) {
        logger.error(i18n.t('failedDelete', { path: dirPath, error: fallbackErr.message }));
      }
    }
  }
  
  function formatBytes(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  }
  
  logger.info(i18n.t('scanning', { path: chalk.cyan(targetPath) }));
  scanDirectory(targetPath);
  
  if (foundPaths.length === 0) {
    logger.info(chalk.yellow(i18n.t('noFound')));
    return { deleted: [], total: 0, totalSize: 0 };
  }
  
  logger.info(i18n.t('foundTotal', { count: chalk.yellow(foundPaths.length) }));
  
  let pathsToDelete = foundPaths;
  
  if (safeMode && !options.silent) {
    pathsToDelete = await showSafeModeList(foundPaths, i18n, options);
    if (pathsToDelete.length === 0) {
      logger.info(chalk.yellow(i18n.t('safeModeCancelled')));
      return { deleted: [], total: 0, totalSize: 0 };
    }
  }
  
  if (progressBar) {
    progressBar.start(pathsToDelete.length, 0, { status: i18n.t('starting') });
  }
  
  pathsToDelete.forEach((dirPath, index) => {
    deleteDirectory(dirPath, index + 1, pathsToDelete.length);
    if (progressBar) {
      progressBar.update(index + 1);
    }
  });
  
  if (progressBar) {
    progressBar.update(pathsToDelete.length, { status: i18n.t('complete') });
    progressBar.stop();
  }
  
  console.log('\n' + chalk.bold.green('═══════════════════════════════════════════════════'));
  console.log(chalk.bold.green('                    ' + i18n.t('summaryTitle') + '                         '));
  console.log(chalk.bold.green('═══════════════════════════════════════════════════'));
  console.log(chalk.white(i18n.t('totalDeleted', { count: chalk.yellow(deletedPaths.length) })));
  if (options.showSize && totalSize > 0) {
    console.log(chalk.white(i18n.t('totalSpaceFreed', { size: chalk.yellow(formatBytes(totalSize)) })));
  }
  console.log(chalk.bold.green('═══════════════════════════════════════════════════\n'));
  
  return {
    deleted: deletedPaths,
    total: deletedPaths.length,
    totalSize: totalSize
  };
}

function showSafeModeList(foundPaths, i18n, options) {
  return new Promise((resolve) => {
    const readline = require('readline');
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    
    console.log('\n' + chalk.bold.cyan('═══════════════════════════════════════════════════'));
    console.log(chalk.bold.cyan('  ' + i18n.t('safeModeTitle')));
    console.log(chalk.bold.cyan('═══════════════════════════════════════════════════\n'));
    
    const pathsWithSize = foundPaths.map((p, index) => {
      let size = 0;
      if (options.showSize) {
        size = getDirectorySizeSync(p);
      }
      return { index: index + 1, path: p, size };
    });
    
    pathsWithSize.forEach(item => {
      const indexStr = chalk.yellow(`[${item.index}]`);
      const pathStr = chalk.cyan(item.path);
      const sizeStr = options.showSize && item.size > 0 
        ? chalk.gray(` (${formatBytes(item.size)})`)
        : '';
      console.log(`${indexStr} ${pathStr}${sizeStr}`);
    });
    
    console.log('');
    
    rl.question(chalk.yellow(i18n.t('safeModePrompt') + ' '), (answer) => {
      rl.close();
      
      answer = answer.trim();
      
      if (answer === 'q' || answer === 'Q') {
        resolve([]);
        return;
      }
      
      if (answer === '') {
        console.log(chalk.green(i18n.t('safeModeAll', { count: foundPaths.length })));
        resolve(foundPaths);
        return;
      }
      
      try {
        const selectedIndices = parseSelection(answer, foundPaths.length);
        const selectedPaths = selectedIndices.map(i => foundPaths[i - 1]);
        console.log(chalk.green(i18n.t('safeModeSelected', { count: selectedPaths.length })));
        resolve(selectedPaths);
      } catch (err) {
        console.log(chalk.red(i18n.t('safeModeInvalidInput')));
        resolve([]);
      }
    });
  });
}

function getDirectorySizeSync(dirPath) {
  let size = 0;
  try {
    const items = fs.readdirSync(dirPath);
    for (const item of items) {
      const fullPath = path.join(dirPath, item);
      try {
        const itemStats = fs.statSync(fullPath);
        if (itemStats.isDirectory()) {
          size += getDirectorySizeSync(fullPath);
        } else {
          size += itemStats.size;
        }
      } catch (err) {
      }
    }
  } catch (err) {
  }
  return size;
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

function parseSelection(input, maxIndex) {
  const indices = new Set();
  const parts = input.split(',');
  
  for (const part of parts) {
    const trimmed = part.trim();
    if (trimmed.includes('-')) {
      const [start, end] = trimmed.split('-').map(s => parseInt(s.trim()));
      if (isNaN(start) || isNaN(end) || start < 1 || end > maxIndex || start > end) {
        throw new Error('Invalid range');
      }
      for (let i = start; i <= end; i++) {
        indices.add(i);
      }
    } else {
      const num = parseInt(trimmed);
      if (isNaN(num) || num < 1 || num > maxIndex) {
        throw new Error('Invalid number');
      }
      indices.add(num);
    }
  }
  
  return Array.from(indices).sort((a, b) => a - b);
}

module.exports = { findAndDeleteNodeModules };
