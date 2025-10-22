const fs = require('fs');
const path = require('path');
const chalk = require('chalk');

class Logger {
  constructor(options = {}) {
    this.logLevel = options.logLevel || 'info';
    this.logFile = options.logFile || null;
    this.silent = options.silent || false;
    this.i18n = options.i18n || null;
    
    this.levels = {
      debug: 0,
      info: 1,
      warn: 2,
      error: 3,
      success: 1
    };
    
    if (this.logFile) {
      const logDir = path.dirname(this.logFile);
      if (!fs.existsSync(logDir)) {
        fs.mkdirSync(logDir, { recursive: true });
      }
    }
  }
  
  shouldLog(level) {
    return this.levels[level] >= this.levels[this.logLevel];
  }
  
  formatMessage(level, message) {
    const timestamp = new Date().toISOString();
    return `[${timestamp}] [${level.toUpperCase()}] ${message}`;
  }
  
  writeToFile(message) {
    if (this.logFile) {
      try {
        fs.appendFileSync(this.logFile, message + '\n', 'utf8');
      } catch (err) {
        console.error(chalk.red(`Failed to write to log file: ${err.message}`));
      }
    }
  }
  
  debug(message) {
    if (!this.shouldLog('debug')) return;
    const formatted = this.formatMessage('debug', message);
    if (!this.silent) {
      console.log(chalk.gray(formatted));
    }
    this.writeToFile(formatted);
  }
  
  info(message) {
    if (!this.shouldLog('info')) return;
    const formatted = this.formatMessage('info', message);
    if (!this.silent) {
      console.log(chalk.blue(formatted));
    }
    this.writeToFile(formatted);
  }
  
  warn(message) {
    if (!this.shouldLog('warn')) return;
    const formatted = this.formatMessage('warn', message);
    if (!this.silent) {
      console.log(chalk.yellow(formatted));
    }
    this.writeToFile(formatted);
  }
  
  error(message) {
    if (!this.shouldLog('error')) return;
    const formatted = this.formatMessage('error', message);
    if (!this.silent) {
      console.log(chalk.red(formatted));
    }
    this.writeToFile(formatted);
  }
  
  success(message) {
    if (!this.shouldLog('success')) return;
    const formatted = this.formatMessage('success', message);
    if (!this.silent) {
      console.log(chalk.green(formatted));
    }
    this.writeToFile(formatted);
  }
}

module.exports = Logger;
