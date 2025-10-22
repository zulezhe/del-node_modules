#!/usr/bin/env node

const { findAndDeleteNodeModules } = require('../index.js');
const path = require('path');
const chalk = require('chalk');
const inquirer = require('inquirer');
const I18n = require('../lib/i18n');

const i18n = new I18n('zh-CN');

async function parseArguments() {
  const args = process.argv.slice(2);
  const options = {
    targetPath: process.cwd(),
    showProgress: true,
    showSize: false,
    logLevel: 'info',
    logFile: null,
    silent: false,
    interactive: false,
    language: 'zh-CN',
    ignore: [],
    safeMode: true
  };
  
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg === '--lang' || arg === '--language') {
      options.language = args[++i];
      i18n.setLanguage(options.language);
    } else if (arg === '--help' || arg === '-h') {
      showHelp();
      process.exit(0);
    } else if (arg === '--no-progress') {
      options.showProgress = false;
    } else if (arg === '--size' || arg === '-s') {
      options.showSize = true;
    } else if (arg === '--log-level' || arg === '-l') {
      options.logLevel = args[++i];
    } else if (arg === '--log-file' || arg === '-f') {
      options.logFile = path.resolve(args[++i]);
    } else if (arg === '--silent') {
      options.silent = true;
    } else if (arg === '--interactive' || arg === '-i') {
      options.interactive = true;
    } else if (arg === '--ignore') {
      options.ignore.push(args[++i]);
    } else if (arg === '--no-safe') {
      options.safeMode = false;
    } else if (!arg.startsWith('-')) {
      options.targetPath = path.resolve(arg);
    }
  }
  
  return options;
}

function showHelp() {
  console.log(chalk.bold.cyan('\n' + i18n.t('helpTitle') + '\n'));
  console.log(chalk.white(i18n.t('helpUsage')));
  console.log(chalk.gray('  ' + i18n.t('helpUsageText') + '\n'));
  console.log(chalk.white(i18n.t('helpOptions')));
  console.log(chalk.yellow('  -h, --help              ') + chalk.gray(i18n.t('helpOptionHelp')));
  console.log(chalk.yellow('  -i, --interactive       ') + chalk.gray(i18n.t('helpOptionInteractive')));
  console.log(chalk.yellow('  -s, --size              ') + chalk.gray(i18n.t('helpOptionSize')));
  console.log(chalk.yellow('  --no-progress           ') + chalk.gray(i18n.t('helpOptionNoProgress')));
  console.log(chalk.yellow('  -l, --log-level <level> ') + chalk.gray(i18n.t('helpOptionLogLevel')));
  console.log(chalk.yellow('  -f, --log-file <path>   ') + chalk.gray(i18n.t('helpOptionLogFile')));
  console.log(chalk.yellow('  --silent                ') + chalk.gray(i18n.t('helpOptionSilent')));
  console.log(chalk.yellow('  --lang, --language <lang>') + chalk.gray(i18n.t('helpOptionLanguage')));
  console.log(chalk.yellow('  --ignore <dir>          ') + chalk.gray(i18n.t('helpOptionIgnore')));
  console.log(chalk.yellow('  --no-safe               ') + chalk.gray(i18n.t('helpOptionNoSafe')));
  console.log(chalk.white('\n' + i18n.t('helpExamples')));
  console.log(chalk.gray('  dnm'));
  console.log(chalk.gray('  dnm /path/to/directory'));
  console.log(chalk.gray('  dnm --interactive'));
  console.log(chalk.gray('  dnm -s --log-file cleanup.log'));
  console.log(chalk.gray('  dnm --lang en-US'));
  console.log(chalk.gray('  dnm --ignore project1 --ignore project2'));
  console.log(chalk.gray('  dnm --no-safe\n'));
}

async function interactiveMode() {
  console.log(chalk.bold.cyan('\n' + i18n.t('interactiveTitle') + '\n'));
  
  const languageAnswer = await inquirer.prompt([
    {
      type: 'list',
      name: 'language',
      message: i18n.language === 'zh-CN' ? '选择语言 / Select language:' : 'Select language / 选择语言:',
      choices: [
        { name: '简体中文 (Simplified Chinese)', value: 'zh-CN' },
        { name: 'English (US)', value: 'en-US' }
      ],
      default: 'zh-CN'
    }
  ]);
  
  i18n.setLanguage(languageAnswer.language);
  
  const answers = await inquirer.prompt([
    {
      type: 'input',
      name: 'targetPath',
      message: i18n.t('promptTargetPath'),
      default: process.cwd(),
      validate: (input) => {
        const fs = require('fs');
        const resolvedPath = path.resolve(input);
        if (!fs.existsSync(resolvedPath)) {
          return i18n.t('pathNotExist');
        }
        if (!fs.statSync(resolvedPath).isDirectory()) {
          return i18n.t('pathNotDirectory');
        }
        return true;
      }
    },
    {
      type: 'input',
      name: 'ignoreDirs',
      message: i18n.t('promptIgnoreDirs'),
      default: ''
    },
    {
      type: 'confirm',
      name: 'safeMode',
      message: i18n.t('promptSafeMode'),
      default: true
    },
    {
      type: 'confirm',
      name: 'showSize',
      message: i18n.t('promptShowSize'),
      default: false
    },
    {
      type: 'confirm',
      name: 'showProgress',
      message: i18n.t('promptShowProgress'),
      default: true
    },
    {
      type: 'list',
      name: 'logLevel',
      message: i18n.t('promptLogLevel'),
      choices: ['debug', 'info', 'warn', 'error'],
      default: 'info'
    },
    {
      type: 'confirm',
      name: 'useLogFile',
      message: i18n.t('promptUseLogFile'),
      default: false
    },
    {
      type: 'input',
      name: 'logFile',
      message: i18n.t('promptLogFile'),
      default: 'del-node-modules.log',
      when: (answers) => answers.useLogFile,
      filter: (input) => path.resolve(input)
    }
  ]);
  
  const options = {
    targetPath: path.resolve(answers.targetPath),
    showProgress: answers.showProgress,
    showSize: answers.showSize,
    logLevel: answers.logLevel,
    logFile: answers.logFile || null,
    silent: false,
    language: languageAnswer.language,
    ignore: answers.ignoreDirs ? answers.ignoreDirs.split(',').map(d => d.trim()).filter(d => d) : [],
    safeMode: answers.safeMode
  };
  
  const confirmAnswer = await inquirer.prompt([
    {
      type: 'confirm',
      name: 'proceed',
      message: chalk.yellow(i18n.t('promptProceed')),
      default: false
    }
  ]);
  
  if (!confirmAnswer.proceed) {
    console.log(chalk.red('\n' + i18n.t('operationCancelled') + '\n'));
    process.exit(0);
  }
  
  return options;
}

async function main() {
  try {
    let options;
    
    if (process.argv.includes('--interactive') || process.argv.includes('-i')) {
      options = await interactiveMode();
    } else {
      options = await parseArguments();
    }
    
    console.log(chalk.bold.magenta('\n╔════════════════════════════════════════════════╗'));
    console.log(chalk.bold.magenta('║     ' + i18n.t('cliTitle') + '       ║'));
    console.log(chalk.bold.magenta('╚════════════════════════════════════════════════╝\n'));
    
    i18n.setLanguage(options.language);
    
    console.log(chalk.cyan(i18n.t('targetDirectory')) + chalk.white(options.targetPath) + '\n');
    
    options.i18n = i18n;
    const result = await findAndDeleteNodeModules(options.targetPath, options);
    
    if (result && result.total > 0) {
      console.log(chalk.bold.green(i18n.t('cleanupSuccess') + '\n'));
      process.exit(0);
    } else {
      console.log(chalk.yellow(i18n.t('nothingToCleanup') + '\n'));
      process.exit(0);
    }
  } catch (err) {
    console.error(chalk.bold.red('\n' + i18n.t('error', { message: err.message }) + '\n'));
    process.exit(1);
  }
}

main();
