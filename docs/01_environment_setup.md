# 第01章：环境设置

## 概述

在开始编写操作系统之前，我们需要设置开发环境。本章将介绍如何配置Rust开发环境，安装必要的工具，并设置RISC-V目标架构。

## 环境要求

### 必需工具
- **Rust工具链** (1.70+)
- **QEMU模拟器** (支持RISC-V64)
- **操作系统**: macOS、Linux或Windows

### 可选工具
- **VS Code** + Rust扩展
- **Git** (用于版本控制)

## 安装步骤

### 1. 安装Rust

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Windows:**
下载并运行 [rustup-init.exe](https://rustup.rs/)

### 2. 安装QEMU

**macOS:**
```bash
brew install qemu
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install qemu-system-misc
```

**Windows:**
下载 [QEMU for Windows](https://www.qemu.org/download/#windows)

### 3. 添加RISC-V目标

```bash
rustup target add riscv64gc-unknown-none-elf
```

### 4. 安装必要组件

```bash
# 安装源码（用于no_std开发）
rustup component add rust-src

# 安装LLVM工具（用于objcopy）
rustup component add llvm-tools-preview
```

## 验证安装

### 检查Rust版本
```bash
rustc --version
cargo --version
```

### 检查QEMU
```bash
qemu-system-riscv64 --version
```

### 检查目标架构
```bash
rustup target list --installed | grep riscv64
```

## 项目结构

创建项目目录结构：

```
rust-os-in-1000-lines/
├── Cargo.toml              # 项目配置
├── .cargo/
│   └── config.toml        # Cargo配置
├── linker.ld              # 链接器脚本
├── src/
│   ├── main.rs            # 内核入口点
│   ├── lib.rs             # 库文件
│   ├── arch/              # 架构相关
│   ├── common/            # 通用工具
│   ├── drivers/           # 设备驱动
│   ├── fs/                # 文件系统
│   ├── kernel/            # 内核核心
│   └── user/              # 用户程序
├── docs/                  # 教程文档
├── Makefile              # 构建脚本
├── run.sh                # 运行脚本
├── debug.sh              # 调试脚本
└── test.sh               # 测试脚本
```

## Cargo.toml配置

```toml
[package]
name = "rust-os-in-1000"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "kernel"
path = "src/main.rs"

[profile.release]
opt-level = "s"   # 优化大小
lto = true        # 链接时优化
codegen-units = 1
panic = "abort"

[profile.dev]
opt-level = "s"
```

## .cargo/config.toml配置

```toml
[build]
target = "riscv64gc-unknown-none-elf"

[target.riscv64gc-unknown-none-elf]
rustflags = [
    "-C", "link-arg=-Tlinker.ld",
    "-C", "link-arg=-nostdlib",
    "-C", "link-arg=-static",
    "-C", "relocation-model=static",
    "-C", "code-model=medium",
]

[env]
RUST_LOG = "debug"
```

## 链接器脚本

创建 `linker.ld` 文件：

```ld
ENTRY(_start)

SECTIONS
{
    . = 0x80000000;
    
    .text : {
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }
    
    .bss : {
        *(.bss .bss.*)
    }
    
    .stack : {
        . = . + 0x10000;
        _stack_top = .;
    }
    
    .heap : {
        . = . + 0x100000;
        _heap_start = .;
    }
}
```

## 测试环境

运行测试脚本验证环境：

```bash
./test.sh
```

如果一切正常，你应该看到：

```
🧪 Testing Rust OS in 1000 Lines...
🔍 Checking toolchain...
🔧 Checking rust-objcopy...
✅ rust-objcopy found: /path/to/rust-objcopy
🎯 Checking RISC-V target...
🔍 Checking code...
🎨 Checking formatting...
🔨 Testing build...
📦 Creating binary image...
📊 Binary size: 27800 bytes
✅ Binary size is reasonable (< 1MB)
✅ All tests passed!
```

## 常见问题

### 问题1：找不到QEMU
```bash
# macOS
brew install qemu

# Ubuntu/Debian
sudo apt install qemu-system-misc
```

### 问题2：找不到RISC-V目标
```bash
rustup target add riscv64gc-unknown-none-elf
```

### 问题3：找不到rust-objcopy
```bash
rustup component add llvm-tools-preview
```

### 问题4：链接错误
检查 `linker.ld` 文件是否正确，确保路径正确。

## 下一步

环境设置完成后，我们可以开始编写第一个内核程序。请继续阅读 [第一个内核](./01_first_kernel.md) 章节。

## 参考资源

- [Rust官方文档](https://doc.rust-lang.org/)
- [RISC-V规范](https://riscv.org/technical/specifications/)
- [QEMU文档](https://qemu.readthedocs.io/)
- [Rust嵌入式开发](https://docs.rust-embedded.org/)
