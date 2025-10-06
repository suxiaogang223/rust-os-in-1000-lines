# Rust OS in 1000 Lines - Makefile
# 用于构建和运行RISC-V64操作系统

# 工具链配置
RUST_TARGET = riscv64gc-unknown-none-elf
QEMU = qemu-system-riscv64
CARGO = cargo

# 构建配置
BUILD_MODE = release
KERNEL_BIN = target/$(RUST_TARGET)/$(BUILD_MODE)/kernel
KERNEL_ELF = $(KERNEL_BIN)
KERNEL_BIN_STRIPPED = $(KERNEL_BIN).stripped

# QEMU配置
QEMU_MACHINE = virt
QEMU_CPU = rv64
QEMU_MEMORY = 128M
QEMU_SMP = 1
QEMU_DISPLAY = -nographic
QEMU_MONITOR = -monitor stdio

# 默认目标
.PHONY: all build run clean check fmt clippy help

all: build

# 构建内核
build: $(KERNEL_BIN_STRIPPED)

$(KERNEL_BIN_STRIPPED): $(KERNEL_ELF)
	@echo "Stripping kernel binary..."
	rust-objcopy --strip-all $(KERNEL_ELF) -O binary $@

$(KERNEL_ELF):
	@echo "Building Rust OS kernel..."
	$(CARGO) build --target $(RUST_TARGET) --$(BUILD_MODE)

# 运行QEMU
run: build
	@echo "Starting QEMU RISC-V64 simulator..."
	$(QEMU) \
		-machine $(QEMU_MACHINE) \
		-cpu $(QEMU_CPU) \
		-m $(QEMU_MEMORY) \
		-smp $(QEMU_SMP) \
		$(QEMU_DISPLAY) \
		$(QEMU_MONITOR) \
		-kernel $(KERNEL_BIN_STRIPPED)

# 调试模式运行
debug: build
	@echo "Starting QEMU in debug mode..."
	$(QEMU) \
		-machine $(QEMU_MACHINE) \
		-cpu $(QEMU_CPU) \
		-m $(QEMU_MEMORY) \
		-smp $(QEMU_SMP) \
		$(QEMU_DISPLAY) \
		$(QEMU_MONITOR) \
		-s -S \
		-kernel $(KERNEL_BIN_STRIPPED)

# 检查代码
check:
	@echo "Checking Rust code..."
	$(CARGO) check --target $(RUST_TARGET)

# 格式化代码
fmt:
	@echo "Formatting Rust code..."
	$(CARGO) fmt

# 代码检查
clippy:
	@echo "Running Clippy..."
	$(CARGO) clippy --target $(RUST_TARGET)

# 清理构建文件
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	rm -f $(KERNEL_BIN_STRIPPED)

# 安装RISC-V工具链
install-toolchain:
	@echo "Installing RISC-V toolchain..."
	rustup target add $(RUST_TARGET)
	rustup component add rust-src
	cargo install cargo-binutils
	rustup component add llvm-tools-preview

# 创建磁盘镜像
disk:
	@echo "Creating disk image..."
	dd if=/dev/zero of=disk.img bs=1M count=64
	mkfs.fat -F 32 disk.img

# 运行带磁盘的QEMU
run-with-disk: build disk
	@echo "Starting QEMU with disk image..."
	$(QEMU) \
		-machine $(QEMU_MACHINE) \
		-cpu $(QEMU_CPU) \
		-m $(QEMU_MEMORY) \
		-smp $(QEMU_SMP) \
		$(QEMU_DISPLAY) \
		$(QEMU_MONITOR) \
		-drive file=disk.img,format=raw,id=hd0 \
		-device virtio-blk-device,drive=hd0 \
		-kernel $(KERNEL_BIN_STRIPPED)

# 运行测试
test:
	@echo "Running tests..."
	$(CARGO) test --target $(RUST_TARGET)

# 生成文档
doc:
	@echo "Generating documentation..."
	$(CARGO) doc --target $(RUST_TARGET) --no-deps

# 显示帮助信息
help:
	@echo "Rust OS in 1000 Lines - Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build          - Build the kernel"
	@echo "  run            - Build and run in QEMU"
	@echo "  debug          - Run in QEMU debug mode"
	@echo "  check          - Check Rust code"
	@echo "  fmt            - Format Rust code"
	@echo "  clippy         - Run Clippy linter"
	@echo "  clean          - Clean build artifacts"
	@echo "  install-toolchain - Install RISC-V toolchain"
	@echo "  disk           - Create disk image"
	@echo "  run-with-disk  - Run with disk image"
	@echo "  test           - Run tests"
	@echo "  doc            - Generate documentation"
	@echo "  help           - Show this help"
	@echo ""
	@echo "Examples:"
	@echo "  make run                    # Build and run"
	@echo "  make debug                  # Run in debug mode"
	@echo "  make clean && make run     # Clean build and run"
