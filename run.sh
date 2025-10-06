#!/bin/bash
# Rust OS in 1000 Lines - Run Script

set -e

echo "🚀 Starting Rust OS in 1000 Lines..."

# 检查内核文件是否存在
KERNEL_BIN="target/riscv64gc-unknown-none-elf/release/kernel.stripped"
if [ ! -f "$KERNEL_BIN" ]; then
    echo "❌ Kernel binary not found. Building first..."
    ./build.sh
fi

echo "🖥️  Starting QEMU RISC-V64 simulator..."
echo "   Press Ctrl+A then X to exit QEMU"
echo ""

# 启动QEMU
qemu-system-riscv64 \
    -machine virt \
    -cpu rv64 \
    -m 128M \
    -smp 1 \
    -nographic \
    -kernel "$KERNEL_BIN"
