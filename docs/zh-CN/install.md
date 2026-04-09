# 安装

## 全局安装（推荐）

```bash
npm install -g dnm
```

安装后，`dnm` 命令可在任意位置使用。

## 使用 npx（无需安装）

```bash
npx dnm /path/to/directory
```

## 从源码安装

```bash
git clone https://github.com/yourusername/dnm.git
cd dnm
npm install
npm link
```

这会将 `dnm` 链接到全局 npm bin，可在任何位置运行。

## 验证安装

```bash
dnm --help
```

你将看到包含所有选项的帮助信息。

## 系统要求

- Node.js >= 16.0.0
- npm 或 pnpm
