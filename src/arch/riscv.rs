// RISC-V 架构相关代码
// 第02章：RISC-V 101

// RISC-V 寄存器定义
pub const REG_SP: usize = 2; // 栈指针
pub const REG_RA: usize = 1; // 返回地址

// RISC-V 特权级别
pub const PRIVILEGE_USER: usize = 0;
pub const PRIVILEGE_SUPERVISOR: usize = 1;
pub const PRIVILEGE_MACHINE: usize = 3;

// 系统调用号
pub const SYS_EXIT: usize = 93;
pub const SYS_WRITE: usize = 64;

// 异常类型
pub const EXCEPTION_INSTRUCTION_MISALIGNED: usize = 0;
pub const EXCEPTION_INSTRUCTION_ACCESS_FAULT: usize = 1;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: usize = 2;
pub const EXCEPTION_BREAKPOINT: usize = 3;
pub const EXCEPTION_LOAD_ADDRESS_MISALIGNED: usize = 4;
pub const EXCEPTION_LOAD_ACCESS_FAULT: usize = 5;
pub const EXCEPTION_STORE_ADDRESS_MISALIGNED: usize = 6;
pub const EXCEPTION_STORE_ACCESS_FAULT: usize = 7;
pub const EXCEPTION_ECALL_FROM_USER: usize = 8;
pub const EXCEPTION_ECALL_FROM_SUPERVISOR: usize = 9;
pub const EXCEPTION_ECALL_FROM_MACHINE: usize = 11;

// 中断类型
pub const INTERRUPT_USER_SOFTWARE: usize = 0;
pub const INTERRUPT_SUPERVISOR_SOFTWARE: usize = 1;
pub const INTERRUPT_MACHINE_SOFTWARE: usize = 3;
pub const INTERRUPT_USER_TIMER: usize = 4;
pub const INTERRUPT_SUPERVISOR_TIMER: usize = 5;
pub const INTERRUPT_MACHINE_TIMER: usize = 7;
pub const INTERRUPT_USER_EXTERNAL: usize = 8;
pub const INTERRUPT_SUPERVISOR_EXTERNAL: usize = 9;
pub const INTERRUPT_MACHINE_EXTERNAL: usize = 11;

// 读取和写入控制状态寄存器
#[inline]
pub unsafe fn read_csr(csr: usize) -> usize {
    let value: usize;
    match csr {
        0x300 => core::arch::asm!("csrr {}, mstatus", out(reg) value),
        0x341 => core::arch::asm!("csrr {}, mepc", out(reg) value),
        0x342 => core::arch::asm!("csrr {}, mcause", out(reg) value),
        0x343 => core::arch::asm!("csrr {}, mtval", out(reg) value),
        0x304 => core::arch::asm!("csrr {}, mie", out(reg) value),
        0x344 => core::arch::asm!("csrr {}, mip", out(reg) value),
        0x305 => core::arch::asm!("csrr {}, mtvec", out(reg) value),
        _ => value = 0,
    }
    value
}

#[inline]
pub unsafe fn write_csr(csr: usize, value: usize) {
    match csr {
        0x300 => core::arch::asm!("csrw mstatus, {}", in(reg) value),
        0x341 => core::arch::asm!("csrw mepc, {}", in(reg) value),
        0x304 => core::arch::asm!("csrw mie, {}", in(reg) value),
        0x305 => core::arch::asm!("csrw mtvec, {}", in(reg) value),
        _ => {}
    }
}

// 读取和写入机器状态寄存器
#[inline]
pub unsafe fn read_mstatus() -> usize {
    read_csr(0x300)
}

#[inline]
pub unsafe fn write_mstatus(value: usize) {
    write_csr(0x300, value);
}

// 读取和写入机器异常程序计数器
#[inline]
pub unsafe fn read_mepc() -> usize {
    read_csr(0x341)
}

#[inline]
pub unsafe fn write_mepc(value: usize) {
    write_csr(0x341, value);
}

// 读取和写入机器异常原因
#[inline]
pub unsafe fn read_mcause() -> usize {
    read_csr(0x342)
}

// 读取和写入机器异常值
#[inline]
pub unsafe fn read_mtval() -> usize {
    read_csr(0x343)
}

// 读取和写入机器中断使能
#[inline]
pub unsafe fn read_mie() -> usize {
    read_csr(0x304)
}

#[inline]
pub unsafe fn write_mie(value: usize) {
    write_csr(0x304, value);
}

// 读取和写入机器中断待处理
#[inline]
pub unsafe fn read_mip() -> usize {
    read_csr(0x344)
}

// 读取和写入机器中断向量
#[inline]
pub unsafe fn read_mtvec() -> usize {
    read_csr(0x305)
}

#[inline]
pub unsafe fn write_mtvec(value: usize) {
    write_csr(0x305, value);
}

// 读取和写入机器模式地址转换和保护
#[inline]
pub unsafe fn read_mpp() -> usize {
    (read_mstatus() >> 11) & 0x3
}

#[inline]
pub unsafe fn write_mpp(value: usize) {
    let mstatus = read_mstatus();
    let new_mstatus = (mstatus & !(0x3 << 11)) | ((value & 0x3) << 11);
    write_mstatus(new_mstatus);
}

// 读取和写入机器模式中断使能
#[inline]
pub unsafe fn read_mpie() -> bool {
    (read_mstatus() >> 7) & 1 != 0
}

#[inline]
pub unsafe fn write_mpie(value: bool) {
    let mstatus = read_mstatus();
    let new_mstatus = if value {
        mstatus | (1 << 7)
    } else {
        mstatus & !(1 << 7)
    };
    write_mstatus(new_mstatus);
}

// 读取和写入机器模式全局中断使能
#[inline]
pub unsafe fn read_mie_global() -> bool {
    (read_mstatus() >> 3) & 1 != 0
}

#[inline]
pub unsafe fn write_mie_global(value: bool) {
    let mstatus = read_mstatus();
    let new_mstatus = if value {
        mstatus | (1 << 3)
    } else {
        mstatus & !(1 << 3)
    };
    write_mstatus(new_mstatus);
}
