#!/bin/bash
# 部署教程到GitHub Pages

set -e

echo "📚 部署Rust OS教程到GitHub Pages..."

# 检查是否在正确的目录
if [ ! -f "docs/book.toml" ]; then
    echo "❌ 请在项目根目录运行此脚本"
    exit 1
fi

# 安装mdBook（如果未安装）
if ! command -v mdbook &> /dev/null; then
    echo "📦 安装mdBook..."
    curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz
    sudo mv mdbook /usr/local/bin/
fi

# 构建文档
echo "🔨 构建文档..."
cd docs
mdbook build

# 检查构建结果
if [ ! -d "book" ]; then
    echo "❌ 文档构建失败"
    exit 1
fi

echo "✅ 文档构建成功！"
echo "📁 构建文件位于: docs/book/"
echo ""
echo "🚀 部署步骤："
echo "1. 将 docs/book/ 目录推送到 gh-pages 分支"
echo "2. 在GitHub仓库设置中启用GitHub Pages"
echo "3. 选择 gh-pages 分支作为源"
echo ""
echo "或者使用GitHub Actions自动部署（推荐）"
