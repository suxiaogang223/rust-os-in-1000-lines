#!/bin/bash
# Rust OS 教程部署脚本

set -e

echo "🚀 部署Rust OS教程到GitHub Pages..."

# 检查是否在正确的目录
if [ ! -f "docs/book.toml" ]; then
    echo "❌ 请在项目根目录运行此脚本"
    exit 1
fi

# 检查Git状态
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  检测到未提交的更改，请先提交："
    git status
    echo ""
    read -p "是否继续部署？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ 部署已取消"
        exit 1
    fi
fi

# 检查mdBook是否安装
if ! command -v mdbook &> /dev/null; then
    echo "📦 安装mdBook..."
    cargo install mdbook
fi

# 构建文档
echo "🔨 构建教程文档..."
cd docs
mdbook build

# 检查构建结果
if [ ! -d "book" ]; then
    echo "❌ 文档构建失败"
    exit 1
fi

echo "✅ 文档构建成功！"

# 检查GitHub Pages设置
echo "🔍 检查GitHub Pages设置..."
if ! git remote get-url origin &> /dev/null; then
    echo "❌ 未找到Git远程仓库"
    exit 1
fi

# 显示部署信息
echo ""
echo "📊 部署信息："
echo "   仓库: $(git remote get-url origin)"
echo "   分支: $(git branch --show-current)"
echo "   构建目录: docs/book/"
echo ""

# 选择部署方式
echo "选择部署方式："
echo "1) 推送到gh-pages分支（手动部署）"
echo "2) 推送到main分支（自动部署）"
echo "3) 仅构建，不部署"
read -p "请选择 (1-3): " choice

case $choice in
    1)
        echo "📤 推送到gh-pages分支..."
        git checkout --orphan gh-pages
        git rm -rf .
        cp -r book/* .
        git add .
        git commit -m "部署教程文档 $(date)"
        git push origin gh-pages
        git checkout main
        echo "✅ 已推送到gh-pages分支"
        echo "🌐 请在GitHub仓库设置中启用GitHub Pages"
        ;;
    2)
        echo "📤 推送到main分支..."
        git add .
        git commit -m "更新教程文档 $(date)"
        git push origin main
        echo "✅ 已推送到main分支"
        echo "🤖 GitHub Actions将自动部署"
        ;;
    3)
        echo "✅ 文档已构建完成"
        echo "📁 构建文件位于: docs/book/"
        ;;
    *)
        echo "❌ 无效选择"
        exit 1
        ;;
esac

echo ""
echo "🎉 部署完成！"
echo ""
echo "📚 教程网站将在以下地址可用："
echo "   https://$(git remote get-url origin | sed 's/.*github.com[:/]\([^/]*\)\/\([^/]*\)\.git.*/\1.github.io\/\2/')"
echo ""
echo "💡 提示："
echo "   - 如果使用自动部署，请等待几分钟"
echo "   - 如果使用手动部署，请在GitHub仓库设置中启用Pages"
echo "   - 本地预览: cd docs && mdbook serve"
