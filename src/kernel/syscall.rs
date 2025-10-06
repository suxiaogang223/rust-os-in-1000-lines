// 第14章：系统调用
// 实现系统调用接口

use crate::common::types::ToString;
use crate::drivers::uart;
use crate::kernel::process;

// 系统调用号
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_GETPID: usize = 172;
pub const SYS_FORK: usize = 220;
pub const SYS_EXECVE: usize = 221;
pub const SYS_WAITPID: usize = 260;

// 系统调用参数
#[derive(Debug)]
pub struct SyscallArgs {
    pub syscall_num: usize,
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub arg4: usize,
    pub arg5: usize,
}

impl SyscallArgs {
    pub fn from_registers() -> Self {
        let (syscall_num, arg0, arg1, arg2, arg3, arg4, arg5): (
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        );

        unsafe {
            core::arch::asm!(
                "mv {}, a7",
                "mv {}, a0",
                "mv {}, a1",
                "mv {}, a2",
                "mv {}, a3",
                "mv {}, a4",
                "mv {}, a5",
                out(reg) syscall_num,
                out(reg) arg0,
                out(reg) arg1,
                out(reg) arg2,
                out(reg) arg3,
                out(reg) arg4,
                out(reg) arg5,
            );
        }

        Self {
            syscall_num,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        }
    }
}

// 系统调用处理器
pub struct SyscallHandler {
    pub syscall_count: usize,
}

impl SyscallHandler {
    pub const fn new() -> Self {
        Self { syscall_count: 0 }
    }

    pub fn handle_syscall(&mut self, args: SyscallArgs) -> usize {
        self.syscall_count += 1;

        uart::print("Syscall #");
        uart::print(&self.syscall_count.to_string());
        uart::print(": ");
        uart::println(&args.syscall_num.to_string());

        match args.syscall_num {
            SYS_WRITE => self.handle_write(args),
            SYS_EXIT => self.handle_exit(args),
            SYS_GETPID => self.handle_getpid(args),
            SYS_FORK => self.handle_fork(args),
            SYS_EXECVE => self.handle_execve(args),
            SYS_WAITPID => self.handle_waitpid(args),
            _ => {
                uart::print("Unknown syscall: ");
                uart::println(&args.syscall_num.to_string());
                0xFFFFFFFF
            }
        }
    }

    // 处理write系统调用
    fn handle_write(&self, args: SyscallArgs) -> usize {
        let fd = args.arg0;
        let buf_ptr = args.arg1 as *const u8;
        let count = args.arg2;

        uart::print("write(fd=");
        uart::print(&fd.to_string());
        uart::print(", count=");
        uart::print(&count.to_string());
        uart::println(")");

        if fd == 1 || fd == 2 {
            // stdout or stderr
            unsafe {
                let slice = core::slice::from_raw_parts(buf_ptr, count);
                for &byte in slice {
                    uart::put_char(byte);
                }
            }
            count
        } else {
            uart::println("Invalid file descriptor");
            0xFFFFFFFF
        }
    }

    // 处理exit系统调用
    fn handle_exit(&self, args: SyscallArgs) -> usize {
        let exit_code = args.arg0;

        uart::print("exit(code=");
        uart::print(&exit_code.to_string());
        uart::println(")");

        // 终止当前进程
        process::terminate_process(0); // 假设PID 0是当前进程

        0
    }

    // 处理getpid系统调用
    fn handle_getpid(&self, _args: SyscallArgs) -> usize {
        uart::println("getpid()");
        // 返回当前进程ID
        1 // 简化实现
    }

    // 处理fork系统调用
    fn handle_fork(&self, _args: SyscallArgs) -> usize {
        uart::println("fork()");

        // 创建新进程
        if let Some(pid) = process::create_process(4096) {
            uart::print("Forked process with PID: ");
            uart::println(&pid.to_string());
            pid
        } else {
            0xFFFFFFFF
        }
    }

    // 处理execve系统调用
    fn handle_execve(&self, args: SyscallArgs) -> usize {
        let pathname = args.arg0 as *const u8;
        let argv = args.arg1 as *const *const u8;
        let envp = args.arg2 as *const *const u8;

        uart::print("execve(pathname=");
        // 这里应该读取pathname字符串，简化处理
        uart::println("program)");

        // 简化实现，不实际执行程序
        0xFFFFFFFF
    }

    // 处理waitpid系统调用
    fn handle_waitpid(&self, args: SyscallArgs) -> usize {
        let pid = args.arg0;
        let status = args.arg1 as *mut i32;
        let options = args.arg2;

        uart::print("waitpid(pid=");
        uart::print(&pid.to_string());
        uart::print(", options=");
        uart::print(&options.to_string());
        uart::println(")");

        // 简化实现
        if !status.is_null() {
            unsafe {
                *status = 0;
            }
        }

        pid
    }
}

// 全局系统调用处理器
pub static mut SYSCALL_HANDLER: SyscallHandler = SyscallHandler::new();

// 全局函数
pub fn init_syscalls() {
    uart::println("System call handler initialized");
}

pub fn handle_syscall(args: SyscallArgs) -> usize {
    unsafe { SYSCALL_HANDLER.handle_syscall(args) }
}

// 系统调用包装函数
pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> usize {
    let args = SyscallArgs {
        syscall_num: SYS_WRITE,
        arg0: fd,
        arg1: buf as usize,
        arg2: count,
        arg3: 0,
        arg4: 0,
        arg5: 0,
    };
    handle_syscall(args)
}

pub fn sys_exit(exit_code: usize) -> usize {
    let args = SyscallArgs {
        syscall_num: SYS_EXIT,
        arg0: exit_code,
        arg1: 0,
        arg2: 0,
        arg3: 0,
        arg4: 0,
        arg5: 0,
    };
    handle_syscall(args)
}

pub fn sys_getpid() -> usize {
    let args = SyscallArgs {
        syscall_num: SYS_GETPID,
        arg0: 0,
        arg1: 0,
        arg2: 0,
        arg3: 0,
        arg4: 0,
        arg5: 0,
    };
    handle_syscall(args)
}
