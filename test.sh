#!/bin/bash
# Rust OS in 1000 Lines - Test Script

set -e

echo "🧪 Testing Rust OS in 1000 Lines..."

# 检查工具链
echo "🔍 Checking toolchain..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found"
    exit 1
fi

if ! command -v qemu-system-riscv64 &> /dev/null; then
    echo "❌ QEMU RISC-V64 not found"
    exit 1
fi

# 检查并安装 rust-objcopy
echo "🔧 Checking rust-objcopy..."
RUST_OBJCOPY=$(find ~/.rustup -name "rust-objcopy" 2>/dev/null | head -1)
if [ -z "$RUST_OBJCOPY" ]; then
    echo "📦 Installing rust-objcopy (llvm-tools-preview)..."
    rustup component add llvm-tools-preview
    RUST_OBJCOPY=$(find ~/.rustup -name "rust-objcopy" 2>/dev/null | head -1)
    if [ -z "$RUST_OBJCOPY" ]; then
        echo "❌ Failed to install rust-objcopy"
        exit 1
    fi
    echo "✅ rust-objcopy installed successfully"
else
    echo "✅ rust-objcopy found: $RUST_OBJCOPY"
fi

# 检查RISC-V目标
echo "🎯 Checking RISC-V target..."
if ! rustup target list --installed | grep -q "riscv64gc-unknown-none-elf"; then
    echo "📦 Adding RISC-V target..."
    rustup target add riscv64gc-unknown-none-elf
fi

# 检查代码
echo "🔍 Checking code..."
cargo check --target riscv64gc-unknown-none-elf

# 格式化检查
echo "🎨 Checking formatting..."
cargo fmt -- --check

# 构建测试
echo "🔨 Testing build..."
cargo build --target riscv64gc-unknown-none-elf --release

# 检查二进制文件
KERNEL_BIN="target/riscv64gc-unknown-none-elf/release/kernel"
if [ ! -f "$KERNEL_BIN" ]; then
    echo "❌ Kernel binary not found"
    exit 1
fi

# 生成二进制文件
echo "📦 Creating binary image..."
"$RUST_OBJCOPY" --strip-all "$KERNEL_BIN" -O binary "${KERNEL_BIN}.stripped"

# 检查二进制文件大小
BIN_SIZE=$(stat -f%z "${KERNEL_BIN}.stripped" 2>/dev/null || stat -c%s "${KERNEL_BIN}.stripped" 2>/dev/null)
echo "📊 Binary size: $BIN_SIZE bytes"

if [ "$BIN_SIZE" -lt 1000000 ]; then
    echo "✅ Binary size is reasonable (< 1MB)"
else
    echo "⚠️  Binary size is large (> 1MB)"
fi

echo "✅ All tests passed!"
echo ""
echo "🚀 Ready to run:"
echo "   make run"
echo "   ./run.sh"
