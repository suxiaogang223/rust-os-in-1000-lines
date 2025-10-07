# 教程部署指南

## 概述

本教程使用 [mdBook](https://rust-lang.github.io/mdBook/) 生成静态网站，并部署到 GitHub Pages。

## 部署方式

### 方式1：GitHub Actions（推荐）

1. **启用GitHub Pages**
   - 进入仓库设置 → Pages
   - 选择 "GitHub Actions" 作为源

2. **自动部署**
   - 推送代码到 `main` 分支
   - GitHub Actions 会自动构建并部署

### 方式2：手动部署

1. **构建文档**
   ```bash
   ./deploy-docs.sh
   ```

2. **推送到gh-pages分支**
   ```bash
   git subtree push --prefix docs/book origin gh-pages
   ```

3. **启用GitHub Pages**
   - 进入仓库设置 → Pages
   - 选择 `gh-pages` 分支作为源

## 本地预览

```bash
# 安装mdBook
cargo install mdbook

# 进入docs目录
cd docs

# 启动本地服务器
mdbook serve

# 打开浏览器访问 http://localhost:3000
```

## 文件结构

```
docs/
├── README.md              # 教程首页
├── SUMMARY.md             # 目录结构
├── book.toml             # mdBook配置
├── 01_environment_setup.md
├── 01_first_kernel.md
├── 02_riscv_basics.md
└── ...
```

## 配置说明

### book.toml
```toml
[book]
title = "Rust OS in 1000 Lines"
description = "一个用Rust语言实现的1000行代码操作系统教程"
authors = ["Rust OS Tutorial"]
language = "zh"

[output.html]
default-theme = "light"
git-repository-url = "https://github.com/your-username/rust-os-in-1000-lines"
```

## 自定义配置

### 主题
- 默认主题：light
- 暗色主题：navy
- 可在 `book.toml` 中修改

### 搜索
- 已启用全文搜索
- 支持中文搜索
- 结果限制：30条

### 编辑链接
- 自动生成编辑链接
- 指向GitHub仓库
- 可在 `book.toml` 中配置

## 故障排除

### 问题1：构建失败
```bash
# 检查mdBook版本
mdbook --version

# 更新mdBook
cargo install mdbook --force
```

### 问题2：样式问题
- 检查 `book.toml` 配置
- 确保主题设置正确
- 清除浏览器缓存

### 问题3：链接错误
- 检查 `SUMMARY.md` 中的链接
- 确保文件路径正确
- 使用相对路径

## 更新教程

1. **修改内容**
   - 编辑对应的 `.md` 文件
   - 更新 `SUMMARY.md` 如果需要

2. **本地测试**
   ```bash
   cd docs
   mdbook serve
   ```

3. **提交更改**
   ```bash
   git add .
   git commit -m "更新教程内容"
   git push
   ```

4. **自动部署**
   - GitHub Actions 会自动构建并部署
   - 等待几分钟后访问网站

## 性能优化

### 图片优化
- 使用 WebP 格式
- 压缩图片大小
- 使用适当的尺寸

### 代码高亮
- 使用 Rust 语法高亮
- 避免过长的代码块
- 添加适当的注释

### 搜索优化
- 使用描述性的标题
- 添加关键词
- 保持内容结构化

## 监控和分析

### GitHub Pages 统计
- 在仓库设置中查看访问统计
- 监控部署状态
- 检查构建日志

### 用户反馈
- 通过 GitHub Issues 收集反馈
- 定期更新内容
- 响应社区需求

## 最佳实践

1. **内容组织**
   - 保持章节结构清晰
   - 使用一致的格式
   - 添加适当的链接

2. **代码示例**
   - 确保代码可运行
   - 添加详细注释
   - 提供完整示例

3. **文档维护**
   - 定期更新内容
   - 修复错误链接
   - 响应社区反馈

4. **版本控制**
   - 使用语义化版本
   - 记录重要更改
   - 保持向后兼容
