# 🦀 Rust OS in 1000 Lines

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![RISC-V](https://img.shields.io/badge/RISC--V-000000?style=for-the-badge&logo=risc-v&logoColor=white)
![QEMU](https://img.shields.io/badge/QEMU-000000?style=for-the-badge&logo=qemu&logoColor=white)
![GitHub Pages](https://img.shields.io/badge/GitHub%20Pages-000000?style=for-the-badge&logo=github&logoColor=white)

**一个用Rust语言实现的1000行代码操作系统教程**

*参考了[《1000行代码的操作系统》](https://operating-system-in-1000-lines.vercel.app/zh/)教程*

[![开始学习](https://img.shields.io/badge/开始学习-第01章-blue?style=for-the-badge&logo=book&logoColor=white)](01_environment_setup.md)
[![GitHub仓库](https://img.shields.io/badge/GitHub-仓库-black?style=for-the-badge&logo=github&logoColor=white)](https://github.com/your-username/rust-os-in-1000-lines)

</div>

---

## 🎯 项目简介

这是一个完整的Rust操作系统教程，用1000行代码实现了一个功能完整的操作系统内核。通过学习本教程，你将掌握：

- **Rust系统编程** - 在no_std环境中开发
- **操作系统原理** - 内核、进程、内存管理
- **RISC-V架构** - 寄存器、异常、中断
- **设备驱动** - UART、磁盘、文件系统
- **系统调用** - 用户态与内核态交互

## 📚 教程目录

### 第01章：入门
- [环境设置](./01_environment_setup.md)
- [项目结构](./01_project_structure.md)
- [第一个内核](./01_first_kernel.md)

### 第02章：RISC-V 101
- [RISC-V架构基础](./02_riscv_basics.md)
- [寄存器操作](./02_registers.md)
- [控制状态寄存器](./02_csr.md)

### 第03章：UART驱动
- [UART原理](./03_uart_principle.md)
- [串口通信](./03_serial_communication.md)
- [驱动实现](./03_uart_driver.md)

### 第04章：内核恐慌
- [异常处理](./04_exception_handling.md)
- [Panic处理](./04_panic_handler.md)
- [错误恢复](./04_error_recovery.md)

### 第05章：异常处理
- [异常类型](./05_exception_types.md)
- [中断处理](./05_interrupt_handling.md)
- [系统调用](./05_syscalls.md)

### 第06章：内存管理
- [内存分配器](./06_memory_allocator.md)
- [堆管理](./06_heap_management.md)
- [内存保护](./06_memory_protection.md)

### 第07章：进程管理
- [进程概念](./07_process_concept.md)
- [进程调度](./07_process_scheduling.md)
- [上下文切换](./07_context_switching.md)

### 第08章：虚拟内存
- [分页机制](./08_paging.md)
- [页表管理](./08_page_tables.md)
- [地址转换](./08_address_translation.md)

### 第09章：用户模式
- [特权级别](./09_privilege_levels.md)
- [用户态切换](./09_user_mode_switch.md)
- [系统调用接口](./09_syscall_interface.md)

### 第10章：系统调用
- [系统调用机制](./10_syscall_mechanism.md)
- [系统调用实现](./10_syscall_implementation.md)
- [用户接口](./10_user_interface.md)

### 第11章：磁盘I/O
- [磁盘驱动](./11_disk_driver.md)
- [块设备操作](./11_block_device.md)
- [I/O调度](./11_io_scheduling.md)

### 第12章：文件系统
- [FAT文件系统](./12_fat_filesystem.md)
- [文件操作](./12_file_operations.md)
- [目录管理](./12_directory_management.md)

### 第13章：Shell应用
- [命令行界面](./13_cli.md)
- [命令解析](./13_command_parsing.md)
- [用户交互](./13_user_interaction.md)

## 🚀 快速开始

### 1. 环境准备

**macOS用户:**
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装QEMU
brew install qemu

# 添加RISC-V目标
rustup target add riscv64gc-unknown-none-elf
```

**Linux用户:**
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装QEMU
sudo apt update
sudo apt install qemu-system-misc

# 添加RISC-V目标
rustup target add riscv64gc-unknown-none-elf
```

### 2. 克隆项目
```bash
git clone https://github.com/your-username/rust-os-in-1000-lines.git
cd rust-os-in-1000-lines
```

### 3. 运行测试
```bash
./test.sh
```

### 4. 启动系统
```bash
./run.sh
```

### 5. 本地预览教程
```bash
cd docs
mdbook serve
# 访问 http://localhost:3000
```

## 🛠️ 技术栈

| 技术 | 版本 | 描述 |
|------|------|------|
| **Rust** | 1.70+ | 系统编程语言 |
| **RISC-V** | 64位 | 开源指令集架构 |
| **QEMU** | 最新 | 硬件模拟器 |
| **mdBook** | 0.4+ | 文档生成器 |
| **GitHub Pages** | - | 静态网站托管 |

## 📖 学习目标

通过本教程，你将学会：

- 🦀 **Rust系统编程** - 在no_std环境中开发
- 🖥️ **操作系统原理** - 内核、进程、内存管理
- 🔧 **RISC-V架构** - 寄存器、异常、中断
- 💾 **设备驱动** - UART、磁盘、文件系统
- 🔄 **系统调用** - 用户态与内核态交互

## 🌟 项目特色

- ✅ **完整实现** - 从引导到用户程序的完整OS
- ✅ **现代语言** - 使用Rust的安全特性
- ✅ **开源架构** - 基于RISC-V开源指令集
- ✅ **详细教程** - 13个章节的完整学习路径
- ✅ **在线访问** - 通过GitHub Pages部署
- ✅ **中文友好** - 适合中文读者学习

## 📊 项目统计

![代码行数](https://img.shields.io/badge/代码行数-1000+-blue?style=flat-square)
![章节数量](https://img.shields.io/badge/章节数量-13-green?style=flat-square)
![支持架构](https://img.shields.io/badge/支持架构-RISC--V64-orange?style=flat-square)
![许可证](https://img.shields.io/badge/许可证-MIT-green?style=flat-square)

## 🤝 贡献指南

我们欢迎各种形式的贡献！

### 贡献方式
- 🐛 **报告Bug** - 通过GitHub Issues
- 💡 **功能建议** - 提出新功能想法
- 📝 **文档改进** - 完善教程内容
- 🔧 **代码贡献** - 提交Pull Request

### 开发流程
1. Fork项目
2. 创建功能分支
3. 提交更改
4. 创建Pull Request

## 📚 相关资源

- [原版教程](https://operating-system-in-1000-lines.vercel.app/zh/) - C语言版本
- [Rust官方文档](https://doc.rust-lang.org/) - Rust学习资源
- [RISC-V规范](https://riscv.org/technical/specifications/) - 架构文档
- [QEMU文档](https://qemu.readthedocs.io/) - 模拟器文档

## 📄 许可证

本项目采用 [MIT License](LICENSE) 许可证。

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给个Star！**

[![GitHub stars](https://img.shields.io/github/stars/your-username/rust-os-in-1000-lines?style=social)](https://github.com/your-username/rust-os-in-1000-lines)
[![GitHub forks](https://img.shields.io/github/forks/your-username/rust-os-in-1000-lines?style=social)](https://github.com/your-username/rust-os-in-1000-lines)

</div>
