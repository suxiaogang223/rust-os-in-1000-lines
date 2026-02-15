# 常见问题

## 1. 构建时报 “target not found” / riscv 目标缺失

确保安装了目标：

```bash
rustup target add riscv64gc-unknown-none-elf
```

本仓库也提供了：

```bash
make install-toolchain
```

## 2. 找不到 rust-objcopy / cargo-binutils

本项目需要 `rust-objcopy` 生成 `kernel.stripped`：

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

如果你使用的是 `build.sh`，脚本也会尝试安装 `cargo-binutils`。

## 3. QEMU 没有输出 / 输出很乱

检查两点：

1. QEMU 是否以 `-nographic` 启动（把串口输出重定向到终端）
2. UART 基址是否匹配 QEMU virt（本仓库使用 `0x1000_0000`）

另外，如果启用了 `-monitor stdio`，monitor 与串口可能复用同一个终端，输出会混在一起；可按需调整启动参数（见 `docs/03_serial_communication.md`）。

## 4. 如何退出 QEMU

常见方式：

- `Ctrl+A`，然后按 `X`

## 5. mdBook 构建失败 / 链接错误

先确认目录文件都存在且 `docs/SUMMARY.md` 中链接正确，然后运行：

```bash
cd docs
mdbook build
```

如果报“missing file”，通常是 `SUMMARY.md` 指向了不存在的 `.md`。

## 6. 为什么很多实现看起来很“简化”

这是学习型项目的常见选择：先把骨架跑通、把机制讲清楚，再逐步替换为更真实的实现（例如 virtio-blk、Sv39 多级页表、真正的用户态返回路径等）。

如果你希望把项目推进到“更真实可用”，建议从以下方向补齐：

- trap 返回路径（`mret`）与现场保存/恢复
- 页表与 `satp` 设置、TLB 刷新
- virtio-blk 驱动与真实 FAT 解析

## 7. 我在 macOS 上没有 gdb-multiarch

你可以使用：

- 安装合适的 GDB（或用 lldb + 插件，但体验可能不同）
- 或在 Linux 环境（容器/虚拟机）里调试

调试流程参考 `Makefile` 的 `debug` 目标与 `debug.sh` 输出提示。
