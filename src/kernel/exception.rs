// 第08章：异常处理
// 实现RISC-V异常和中断处理

use crate::arch::riscv::*;
use crate::common::types::ToString;
use crate::drivers::uart;

// 异常处理函数
#[no_mangle]
pub extern "C" fn handle_exception() {
    let mcause = unsafe { read_mcause() };
    let mepc = unsafe { read_mepc() };
    let mtval = unsafe { read_mtval() };

    // 检查是异常还是中断
    if mcause & (1 << 63) != 0 {
        // 中断
        let interrupt_code = mcause & 0x7FFFFFFF;
        handle_interrupt(interrupt_code);
    } else {
        // 异常
        let exception_code = mcause;
        handle_exception_code(exception_code, mepc, mtval);
    }
}

// 处理中断
fn handle_interrupt(code: usize) {
    match code {
        INTERRUPT_MACHINE_TIMER => {
            uart::println("Timer interrupt");
            // 处理定时器中断
        }
        INTERRUPT_MACHINE_EXTERNAL => {
            uart::println("External interrupt");
            // 处理外部中断
        }
        _ => {
            uart::print("Unknown interrupt: ");
            uart::println(&code.to_string());
        }
    }
}

// 处理异常
fn handle_exception_code(code: usize, mepc: usize, mtval: usize) {
    uart::print("Exception: ");

    match code {
        EXCEPTION_INSTRUCTION_MISALIGNED => {
            uart::println("Instruction address misaligned");
        }
        EXCEPTION_INSTRUCTION_ACCESS_FAULT => {
            uart::println("Instruction access fault");
        }
        EXCEPTION_ILLEGAL_INSTRUCTION => {
            uart::println("Illegal instruction");
        }
        EXCEPTION_BREAKPOINT => {
            uart::println("Breakpoint");
        }
        EXCEPTION_LOAD_ADDRESS_MISALIGNED => {
            uart::println("Load address misaligned");
        }
        EXCEPTION_LOAD_ACCESS_FAULT => {
            uart::println("Load access fault");
        }
        EXCEPTION_STORE_ADDRESS_MISALIGNED => {
            uart::println("Store address misaligned");
        }
        EXCEPTION_STORE_ACCESS_FAULT => {
            uart::println("Store access fault");
        }
        EXCEPTION_ECALL_FROM_USER => {
            uart::println("Environment call from user mode");
            handle_syscall(mepc);
        }
        EXCEPTION_ECALL_FROM_SUPERVISOR => {
            uart::println("Environment call from supervisor mode");
        }
        EXCEPTION_ECALL_FROM_MACHINE => {
            uart::println("Environment call from machine mode");
        }
        _ => {
            uart::print("Unknown exception: ");
            uart::println(&code.to_string());
        }
    }

    uart::print("MEPC: 0x");
    uart::println(&mepc.to_string());
    uart::print("MTVAL: 0x");
    uart::println(&mtval.to_string());
}

// 处理系统调用
fn handle_syscall(mepc: usize) {
    // 从a7寄存器读取系统调用号
    let syscall_num: usize;
    unsafe {
        core::arch::asm!("mv {}, a7", out(reg) syscall_num);
    }

    match syscall_num {
        SYS_WRITE => {
            uart::println("System call: write");
            // 处理write系统调用
        }
        SYS_EXIT => {
            uart::println("System call: exit");
            // 处理exit系统调用
        }
        _ => {
            uart::print("Unknown syscall: ");
            uart::println(&syscall_num.to_string());
        }
    }
}

// 设置异常处理向量
pub fn init_exception_handling() {
    unsafe {
        // 设置异常处理向量地址
        write_mtvec(handle_exception as usize);

        // 启用机器模式中断
        write_mie(read_mie() | (1 << INTERRUPT_MACHINE_TIMER) | (1 << INTERRUPT_MACHINE_EXTERNAL));

        // 启用全局中断
        write_mie_global(true);
    }
}
