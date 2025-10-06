#!/bin/bash
# Rust OS in 1000 Lines - Build Script

set -e

echo "🦀 Building Rust OS in 1000 Lines..."

# 检查工具链
echo "📋 Checking toolchain..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

if ! command -v qemu-system-riscv64 &> /dev/null; then
    echo "❌ QEMU RISC-V64 not found. Please install QEMU."
    echo "   On macOS: brew install qemu"
    echo "   On Ubuntu: sudo apt install qemu-system-misc"
    exit 1
fi

# 添加RISC-V目标
echo "🎯 Adding RISC-V target..."
rustup target add riscv64gc-unknown-none-elf

# 安装必要组件
echo "🔧 Installing required components..."
rustup component add rust-src
cargo install cargo-binutils --quiet || true

# 构建内核
echo "🔨 Building kernel..."
cargo build --target riscv64gc-unknown-none-elf --release

# 生成二进制文件
echo "📦 Creating binary image..."
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/kernel -O binary target/riscv64gc-unknown-none-elf/release/kernel.stripped

echo "✅ Build completed successfully!"
echo ""
echo "🚀 To run the OS:"
echo "   make run"
echo ""
echo "🐛 To run in debug mode:"
echo "   make debug"
