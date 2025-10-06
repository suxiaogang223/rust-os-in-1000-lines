#!/bin/bash
# Rust OS in 1000 Lines - Debug Script

set -e

echo "🐛 Starting Rust OS in debug mode..."

# 检查内核文件是否存在
KERNEL_BIN="target/riscv64gc-unknown-none-elf/release/kernel.stripped"
if [ ! -f "$KERNEL_BIN" ]; then
    echo "❌ Kernel binary not found. Building first..."
    ./build.sh
fi

echo "🔍 Starting QEMU in debug mode..."
echo "   Connect GDB with: gdb-multiarch target/riscv64gc-unknown-none-elf/release/kernel"
echo "   Then run: (gdb) target remote :1234"
echo "   Press Ctrl+C to stop QEMU"
echo ""

# 启动QEMU调试模式
qemu-system-riscv64 \
    -machine virt \
    -cpu rv64 \
    -m 128M \
    -smp 1 \
    -nographic \
    -monitor stdio \
    -s -S \
    -kernel "$KERNEL_BIN"
