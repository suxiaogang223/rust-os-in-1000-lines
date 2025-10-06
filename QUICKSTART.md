# 快速开始指南

## 🚀 5分钟快速体验

### 1. 环境准备

**macOS用户:**
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装QEMU
brew install qemu

# 安装RISC-V工具链
brew install riscv-tools
```

**Linux用户:**
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装QEMU
sudo apt update
sudo apt install qemu-system-misc

# 安装RISC-V工具链
sudo apt install gcc-riscv64-unknown-elf
```

### 2. 一键运行

```bash
# 克隆项目（如果还没有）
git clone <your-repo-url>
cd rust-os-in-1000-lines

# 一键构建和运行
make run
```

### 3. 预期输出

你应该看到类似以下的输出：

```
🦀 Building Rust OS kernel...
📦 Creating binary image...
🚀 Starting QEMU RISC-V64 simulator...

Hello, Rust OS!
Welcome to OS in 1000 Lines!
Exception handling initialized
Memory allocator initialized: 0x20000000 - 0x20100000
Memory allocated at: 0x20000000
Process scheduler initialized
Created process with PID: 1
Created process with PID: 2
Process List:
  PID 1: Ready (SP: 0x20001000)
  PID 2: Ready (SP: 0x20002000)
Paging initialized successfully
Mapping: 0x10000000 -> 0x20000000 (4096 bytes)
Unmapping: 0x10000000 (4096 bytes)
User mode manager initialized
Available user programs:
  0: hello
  1: test
System call handler initialized
Disk driver initialized successfully
Disk read test successful
File system initialized successfully
Directory listing:
  0: HELLO    TXT (13 bytes, FILE)
  1: TEST     DIR (0 bytes, DIR)
Read 13 bytes from file
Starting shell...

=== Rust OS in 1000 Lines - 结语 ===

恭喜！你已经成功实现了一个用Rust编写的操作系统内核。

本项目的亮点：
1. 使用Rust语言的安全特性
2. 模块化的代码组织
3. 完整的操作系统功能
4. 清晰的架构设计

实现的功能包括：
- 引导和初始化
- UART串口驱动
- 异常和中断处理
- 内存管理
- 进程调度
- 虚拟内存管理
- 用户态支持
- 系统调用接口
- 磁盘I/O驱动
- FAT文件系统
- Shell应用程序

Rust语言的优势：
- 内存安全：编译时保证无缓冲区溢出
- 零成本抽象：高级特性无运行时开销
- 所有权系统：自动内存管理
- 模式匹配：强大的控制流处理
- 类型安全：编译时类型检查

感谢使用Rust OS in 1000 Lines！
Happy OS hacking with Rust! 🦀
```

### 4. 退出QEMU

按 `Ctrl+A` 然后按 `X` 退出QEMU模拟器。

### 5. 其他命令

```bash
# 调试模式
make debug

# 清理构建文件
make clean

# 代码检查
make check

# 格式化代码
make fmt

# 查看所有可用命令
make help
```

## 🔧 故障排除

### 问题1：找不到QEMU
```bash
# macOS
brew install qemu

# Ubuntu/Debian
sudo apt install qemu-system-misc
```

### 问题2：找不到RISC-V工具链
```bash
# 添加RISC-V目标
rustup target add riscv64gc-unknown-none-elf

# 安装必要组件
rustup component add rust-src
cargo install cargo-binutils
```

### 问题3：构建失败
```bash
# 清理并重新构建
make clean
make build
```

### 问题4：QEMU启动失败
```bash
# 检查QEMU版本
qemu-system-riscv64 --version

# 手动运行
qemu-system-riscv64 -machine virt -cpu rv64 -m 128M -nographic -kernel target/riscv64gc-unknown-none-elf/release/kernel.stripped
```

## 📚 学习资源

- [Rust官方文档](https://doc.rust-lang.org/)
- [RISC-V规范](https://riscv.org/technical/specifications/)
- [QEMU文档](https://qemu.readthedocs.io/)
- [操作系统概念](https://os.phil-opp.com/)

## 🎯 下一步

1. 阅读源代码，理解每个模块的功能
2. 尝试修改代码，添加新功能
3. 学习Rust系统编程
4. 探索操作系统原理
5. 贡献代码和改进

Happy coding! 🦀
