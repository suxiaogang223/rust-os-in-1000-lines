// 第10章：进程管理
// 实现简单的进程调度

use crate::common::types::ToString;
use crate::drivers::uart;
use crate::kernel::memory;

// 进程状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

// 进程控制块
#[derive(Clone, Copy)]
pub struct Process {
    pub pid: usize,
    pub state: ProcessState,
    pub stack_pointer: usize,
    pub program_counter: usize,
    pub stack: *mut u8,
    pub stack_size: usize,
}

impl Process {
    pub fn new(pid: usize, stack_size: usize) -> Option<Self> {
        if let Some(stack_ptr) = memory::allocate(stack_size) {
            Some(Self {
                pid,
                state: ProcessState::Ready,
                stack_pointer: unsafe { stack_ptr.add(stack_size) } as usize,
                program_counter: 0,
                stack: stack_ptr,
                stack_size,
            })
        } else {
            None
        }
    }

    pub fn destroy(&mut self) {
        if !self.stack.is_null() {
            memory::deallocate(self.stack, self.stack_size);
            self.stack = core::ptr::null_mut();
        }
        self.state = ProcessState::Terminated;
    }
}

// 进程调度器
pub struct Scheduler {
    processes: [Option<Process>; 16], // 最多16个进程
    current_pid: Option<usize>,
    next_pid: usize,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            processes: [None; 16],
            current_pid: None,
            next_pid: 1,
        }
    }

    // 创建新进程
    pub fn create_process(&mut self, stack_size: usize) -> Option<usize> {
        for i in 0..self.processes.len() {
            if self.processes[i].is_none() {
                if let Some(process) = Process::new(self.next_pid, stack_size) {
                    self.processes[i] = Some(process);
                    let pid = self.next_pid;
                    self.next_pid += 1;
                    uart::print("Process created: PID ");
                    uart::println(&pid.to_string());
                    return Some(pid);
                }
            }
        }
        None
    }

    // 终止进程
    pub fn terminate_process(&mut self, pid: usize) -> bool {
        for i in 0..self.processes.len() {
            if let Some(process) = &mut self.processes[i] {
                if process.pid == pid {
                    process.destroy();
                    self.processes[i] = None;
                    if self.current_pid == Some(pid) {
                        self.current_pid = None;
                    }
                    uart::print("Process terminated: PID ");
                    uart::println(&pid.to_string());
                    return true;
                }
            }
        }
        false
    }

    // 调度下一个进程
    pub fn schedule(&mut self) -> Option<usize> {
        // 简单的轮转调度
        let start = if let Some(current) = self.current_pid {
            (current % self.processes.len()) + 1
        } else {
            0
        };

        for i in 0..self.processes.len() {
            let index = (start + i) % self.processes.len();
            if let Some(process) = &self.processes[index] {
                if process.state == ProcessState::Ready {
                    // 设置当前进程为就绪状态
                    if let Some(current) = self.current_pid {
                        for j in 0..self.processes.len() {
                            if let Some(p) = &mut self.processes[j] {
                                if p.pid == current {
                                    p.state = ProcessState::Ready;
                                    break;
                                }
                            }
                        }
                    }

                    // 设置新进程为运行状态
                    if let Some(p) = &mut self.processes[index] {
                        p.state = ProcessState::Running;
                        self.current_pid = Some(p.pid);
                        return Some(p.pid);
                    }
                }
            }
        }
        None
    }

    // 获取进程
    pub fn get_process(&self, pid: usize) -> Option<&Process> {
        for process in &self.processes {
            if let Some(p) = process {
                if p.pid == pid {
                    return Some(p);
                }
            }
        }
        None
    }

    // 获取可变进程
    pub fn get_process_mut(&mut self, pid: usize) -> Option<&mut Process> {
        for i in 0..self.processes.len() {
            if let Some(p) = &self.processes[i] {
                if p.pid == pid {
                    return self.processes[i].as_mut();
                }
            }
        }
        None
    }

    // 列出所有进程
    pub fn list_processes(&self) {
        uart::println("Process List:");
        for process in &self.processes {
            if let Some(p) = process {
                uart::print("PID ");
                uart::print(&p.pid.to_string());
                uart::print(": ");
                uart::print(match p.state {
                    ProcessState::Ready => "Ready",
                    ProcessState::Running => "Running",
                    ProcessState::Blocked => "Blocked",
                    ProcessState::Terminated => "Terminated",
                });
                uart::print(" (SP: 0x");
                uart::print(&p.stack_pointer.to_string());
                uart::println(")");
            }
        }
    }
}

// 全局调度器
pub static mut SCHEDULER: Scheduler = Scheduler::new();

// 全局函数
pub fn init_scheduler() {
    uart::println("Process scheduler initialized");
}

pub fn create_process(stack_size: usize) -> Option<usize> {
    unsafe { SCHEDULER.create_process(stack_size) }
}

pub fn terminate_process(pid: usize) -> bool {
    unsafe { SCHEDULER.terminate_process(pid) }
}

pub fn schedule() -> Option<usize> {
    unsafe { SCHEDULER.schedule() }
}

pub fn list_processes() {
    unsafe {
        SCHEDULER.list_processes();
    }
}
