// 第13章：用户模式
// 实现用户态和内核态切换

use crate::arch::riscv::*;
use crate::common::types::ToString;
use crate::drivers::uart;

// 用户态上下文
#[derive(Debug, Clone)]
pub struct UserContext {
    pub regs: [usize; 32], // 32个通用寄存器
    pub pc: usize,         // 程序计数器
    pub sp: usize,         // 栈指针
}

impl UserContext {
    pub fn new(pc: usize, sp: usize) -> Self {
        let mut regs = [0; 32];
        regs[REG_SP] = sp; // 设置栈指针

        Self { regs, pc, sp }
    }

    pub fn set_register(&mut self, reg: usize, value: usize) {
        if reg < 32 {
            self.regs[reg] = value;
        }
    }

    pub fn get_register(&self, reg: usize) -> usize {
        if reg < 32 {
            self.regs[reg]
        } else {
            0
        }
    }
}

// 用户态程序
#[derive(Clone, Copy)]
pub struct UserProgram {
    pub name: &'static str,
    pub entry_point: usize,
    pub stack_size: usize,
}

impl UserProgram {
    pub const fn new(name: &'static str, entry_point: usize, stack_size: usize) -> Self {
        Self {
            name,
            entry_point,
            stack_size,
        }
    }

    pub fn create_context(&self) -> UserContext {
        UserContext::new(self.entry_point, 0x10000000) // 假设用户栈在0x10000000
    }
}

// 用户态管理器
pub struct UserModeManager {
    pub current_user_context: Option<UserContext>,
    pub user_programs: [Option<UserProgram>; 8],
}

impl UserModeManager {
    pub const fn new() -> Self {
        Self {
            current_user_context: None,
            user_programs: [None; 8],
        }
    }

    pub fn init(&mut self) {
        uart::println("User mode manager initialized");

        // 注册一些用户程序
        self.user_programs[0] = Some(UserProgram::new("hello", 0x1000000, 4096));
        self.user_programs[1] = Some(UserProgram::new("test", 0x1001000, 4096));
    }

    // 切换到用户态
    pub fn switch_to_user(&mut self, program_index: usize) -> bool {
        if let Some(program) = &self.user_programs[program_index] {
            uart::print("Switching to user mode: ");
            uart::println(program.name);

            let context = program.create_context();
            self.current_user_context = Some(context);

            // 设置用户态权限
            unsafe {
                // 设置MPP为用户态
                write_mpp(PRIVILEGE_USER);

                // 设置用户态程序计数器
                write_mepc(program.entry_point);

                // 启用用户态中断
                write_mie(
                    read_mie() | (1 << INTERRUPT_USER_SOFTWARE) | (1 << INTERRUPT_USER_TIMER),
                );
            }

            true
        } else {
            uart::println("Invalid program index");
            false
        }
    }

    // 从用户态返回内核态
    pub fn return_to_kernel(&mut self) {
        uart::println("Returning to kernel mode");

        // 清除用户态上下文
        self.current_user_context = None;

        // 设置内核态权限
        unsafe {
            // 设置MPP为机器态
            write_mpp(PRIVILEGE_MACHINE);

            // 禁用用户态中断
            write_mie(read_mie() & !((1 << INTERRUPT_USER_SOFTWARE) | (1 << INTERRUPT_USER_TIMER)));
        }
    }

    // 处理用户态异常
    pub fn handle_user_exception(&mut self, exception_code: usize, mepc: usize, mtval: usize) {
        uart::print("User mode exception: ");

        match exception_code {
            EXCEPTION_ECALL_FROM_USER => {
                uart::println("System call from user mode");
                self.handle_user_syscall(mepc);
            }
            EXCEPTION_ILLEGAL_INSTRUCTION => {
                uart::println("Illegal instruction in user mode");
                self.return_to_kernel();
            }
            EXCEPTION_LOAD_ACCESS_FAULT => {
                uart::println("Load access fault in user mode");
                self.return_to_kernel();
            }
            EXCEPTION_STORE_ACCESS_FAULT => {
                uart::println("Store access fault in user mode");
                self.return_to_kernel();
            }
            _ => {
                uart::print("Unknown user exception: ");
                uart::println(&exception_code.to_string());
                self.return_to_kernel();
            }
        }
    }

    // 处理用户态系统调用
    fn handle_user_syscall(&mut self, mepc: usize) {
        // 从a7寄存器读取系统调用号
        let syscall_num: usize;
        unsafe {
            core::arch::asm!("mv {}, a7", out(reg) syscall_num);
        }

        uart::print("User syscall: ");
        uart::println(&syscall_num.to_string());

        match syscall_num {
            SYS_WRITE => {
                uart::println("User program called write()");
                // 处理write系统调用
            }
            SYS_EXIT => {
                uart::println("User program called exit()");
                self.return_to_kernel();
            }
            _ => {
                uart::print("Unknown user syscall: ");
                uart::println(&syscall_num.to_string());
            }
        }
    }

    // 列出用户程序
    pub fn list_programs(&self) {
        uart::println("Available user programs:");
        for (i, program) in self.user_programs.iter().enumerate() {
            if let Some(prog) = program {
                uart::print("  ");
                uart::print(&i.to_string());
                uart::print(": ");
                uart::println(prog.name);
            }
        }
    }
}

// 全局用户态管理器
pub static mut USER_MODE_MANAGER: UserModeManager = UserModeManager::new();

// 全局函数
pub fn init_user_mode() {
    unsafe {
        USER_MODE_MANAGER.init();
    }
}

pub fn switch_to_user(program_index: usize) -> bool {
    unsafe { USER_MODE_MANAGER.switch_to_user(program_index) }
}

pub fn return_to_kernel() {
    unsafe {
        USER_MODE_MANAGER.return_to_kernel();
    }
}

pub fn handle_user_exception(exception_code: usize, mepc: usize, mtval: usize) {
    unsafe {
        USER_MODE_MANAGER.handle_user_exception(exception_code, mepc, mtval);
    }
}

pub fn list_user_programs() {
    unsafe {
        USER_MODE_MANAGER.list_programs();
    }
}
