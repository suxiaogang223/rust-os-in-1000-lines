# Rust OS in 1000 Lines

一个用Rust实现的1000行代码操作系统，参考了[《1000行代码的操作系统》](https://operating-system-in-1000-lines.vercel.app/zh/)教程。

## 项目概述

这个项目展示了如何用Rust语言从零开始构建一个小型操作系统内核。参考了[《1000行代码的操作系统》](https://operating-system-in-1000-lines.vercel.app/zh/)教程，用Rust重新实现，用于学习Rust系统编程。

## 项目结构

```
src/
├── main.rs              # 内核入口点
├── lib.rs               # 库文件
├── arch/                # 架构相关代码
│   ├── mod.rs
│   └── riscv.rs         # RISC-V架构支持
├── common/              # 通用工具
│   └── mod.rs
├── drivers/             # 设备驱动
│   ├── mod.rs
│   ├── uart.rs          # UART串口驱动
│   └── disk.rs          # 磁盘驱动
├── fs/                  # 文件系统
│   ├── mod.rs
│   └── fat.rs           # FAT文件系统
├── kernel/              # 内核核心
│   ├── mod.rs           # 内核主模块
│   ├── exception.rs     # 异常处理
│   ├── memory.rs        # 内存管理
│   ├── process.rs       # 进程管理
│   ├── paging.rs        # 虚拟内存
│   ├── usermode.rs      # 用户态管理
│   └── syscall.rs       # 系统调用
└── user/                 # 用户程序
    ├── mod.rs
    └── shell.rs         # Shell应用
```

## 实现的功能

### 第01章：入门
- 设置Rust no_std环境
- 基本项目结构

### 第02章：RISC-V 101
- RISC-V架构支持
- 寄存器操作
- 控制状态寄存器(CSR)操作

### 第03章：总览
- 整体架构设计
- 模块化组织

### 第04章：引导
- 内核入口点
- 引导过程

### 第05章：Hello World
- UART串口驱动
- 基本输出功能

### 第06章：C标准库
- 基础库函数实现
- 字符串处理

### 第07章：内核恐慌
- Panic处理函数
- 错误信息输出

### 第08章：异常
- 异常处理机制
- 中断处理
- 系统调用处理

### 第09章：内存分配
- 简单内存分配器
- 堆内存管理

### 第10章：进程
- 进程控制块(PCB)
- 进程调度器
- 进程创建和销毁

### 第11章：页表
- 虚拟内存管理
- 页表操作
- 内存映射

### 第12章：应用程序
- Shell应用
- 用户程序框架

### 第13章：用户模式
- 用户态和内核态切换
- 权限管理

### 第14章：系统调用
- 系统调用接口
- 系统调用处理

### 第15章：磁盘I/O
- 磁盘驱动
- 块设备操作

### 第16章：文件系统
- FAT文件系统
- 文件操作

## 技术特点

### Rust语言优势
1. **内存安全**：编译时保证内存安全，避免缓冲区溢出等常见问题
2. **零成本抽象**：高级抽象不带来运行时开销
3. **模式匹配**：强大的控制流处理
4. **所有权系统**：自动内存管理，无需垃圾回收

### 系统编程特性
1. **no_std环境**：不依赖标准库，适合内核开发
2. **内联汇编**：直接操作硬件寄存器
3. **裸指针操作**：底层内存操作
4. **中断处理**：异常和中断处理

## 编译和运行

### 环境要求
- Rust工具链 (1.70+)
- QEMU模拟器 (支持RISC-V64)
- 操作系统：macOS、Linux或Windows

### 快速开始

#### 1. 安装依赖

**macOS:**
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装QEMU
brew install qemu

# 安装RISC-V工具链
brew install riscv-tools
```

**Ubuntu/Debian:**
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装QEMU
sudo apt update
sudo apt install qemu-system-misc

# 安装RISC-V工具链
sudo apt install gcc-riscv64-unknown-elf
```

#### 2. 构建和运行

**使用Makefile (推荐):**
```bash
# 安装工具链
make install-toolchain

# 构建内核
make build

# 运行OS
make run

# 调试模式
make debug

# 清理
make clean
```

**使用脚本:**
```bash
# 构建
./build.sh

# 运行
./run.sh

# 调试
./debug.sh
```

**手动构建:**
```bash
# 添加RISC-V目标
rustup target add riscv64gc-unknown-none-elf

# 构建内核
cargo build --target riscv64gc-unknown-none-elf --release

# 生成二进制文件
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/kernel -O binary target/riscv64gc-unknown-none-elf/release/kernel.stripped

# 运行
qemu-system-riscv64 -machine virt -cpu rv64 -m 128M -nographic -kernel target/riscv64gc-unknown-none-elf/release/kernel.stripped
```

### 可用命令

| 命令 | 描述 |
|------|------|
| `make build` | 构建内核 |
| `make run` | 构建并运行 |
| `make debug` | 调试模式运行 |
| `make check` | 检查代码 |
| `make fmt` | 格式化代码 |
| `make clippy` | 代码检查 |
| `make clean` | 清理构建文件 |
| `make help` | 显示帮助 |

## 学习价值

这个项目展示了：

1. **操作系统基础概念**：进程、内存、文件系统等
2. **Rust系统编程**：no_std、内联汇编、裸指针等
3. **RISC-V架构**：寄存器、异常、中断等
4. **模块化设计**：清晰的代码组织结构

## 参考资源

- [《1000行代码的操作系统》](https://operating-system-in-1000-lines.vercel.app/zh/) - 原教程
- [Rust嵌入式编程](https://docs.rust-embedded.org/) - Rust嵌入式开发指南
- [RISC-V规范](https://riscv.org/technical/specifications/) - RISC-V架构规范

## 许可证

本项目基于MIT许可证开源。
