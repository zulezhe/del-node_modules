# 贡献指南

欢迎贡献！以下是参与方式。

## 开发环境搭建

```bash
# 克隆仓库
git clone https://github.com/yourusername/dnm.git
cd dnm

# 安装依赖
npm install

# 链接到本地开发
npm link

# 运行测试
npm test
```

## 开发流程

1. Fork 本仓库
2. 创建特性分支（`git checkout -b feature/amazing-feature`）
3. 进行更改
4. 运行测试（`npm test`）
5. 提交（`git commit -m 'Add amazing feature'`）
6. 推送（`git push origin feature/amazing-feature`）
7. 发起 Pull Request

## 代码规范

- 遵循现有代码风格
- 保持函数简洁聚焦
- 为新功能添加测试
- 更新文档

## 报告问题

- 使用 [GitHub Issues](https://github.com/yourusername/dnm/issues)
- 包含 Node.js 版本和操作系统信息
- 提供复现步骤

## 添加翻译

1. 打开 `lib/i18n.js`
2. 按照现有结构添加新的语言对象
3. 确保所有键都已翻译
4. 提交 PR

## 许可证

通过贡献代码，你同意你的贡献将在 MIT 许可证下授权。
