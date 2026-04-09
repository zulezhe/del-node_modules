# dnm (Delete Node Modules)

> 一个强大的 Node.js CLI 工具，用于递归查找并删除所有 `node_modules` 目录。

[![CI/CD](https://github.com/yourusername/dnm/workflows/CI/CD%20Pipeline/badge.svg)](https://github.com/yourusername/dnm/actions)
[![npm version](https://badge.fury.io/js/dnm.svg)](https://www.npmjs.com/package/dnm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 为什么选择 dnm？

在多个 Node.js 项目中工作时，`node_modules` 目录可能占用大量磁盘空间。`dnm` 帮助你快速、安全地回收这些空间。

## 核心特性

- **递归扫描** — 查找嵌套目录中的所有 `node_modules`
- **安全模式（默认）** — 删除前先展示列表供确认
- **交互模式** — 引导式向导，适合首次使用
- **进度条** — 删除时实时显示进度
- **彩色输出** — 使用 chalk 美化终端输出
- **多语言** — 支持中文（zh-CN）和英文（en-US）
- **目录大小** — 可选的大小计算和显示
- **忽略模式** — 跳过指定目录
- **日志系统** — 可配置级别，输出到文件
- **跨平台** — 支持 Windows、macOS 和 Linux

## 快速示例

```bash
# 扫描并删除当前目录下的 node_modules
dnm

# 显示目录大小
dnm -s

# 使用英文界面
dnm --lang en-US
```

## 安全第一

`dnm` 默认以**安全模式**运行 — 先扫描并显示所有找到的 `node_modules` 目录，然后让你确认后再删除。你可以通过序号选择要删除的目录（如 `1-3,7`），或按 Enter 删除全部。

## 安装

```bash
npm install -g dnm
```

详见 [安装指南](/zh-CN/install)。
