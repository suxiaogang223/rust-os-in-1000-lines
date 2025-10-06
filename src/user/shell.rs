// 第12章：应用程序
// 实现简单的shell

use crate::common::types::ToString;
use crate::drivers::uart;
use crate::kernel::process;

// Shell命令
#[derive(Debug, Clone)]
pub enum Command {
    Help,
    List,
    Create,
    Exit,
    Unknown(&'static str),
}

impl Command {
    pub fn parse(input: &str) -> Command {
        // 简化实现，直接匹配命令
        match input.trim() {
            "help" => Command::Help,
            "list" => Command::List,
            "create" => Command::Create,
            "exit" => Command::Exit,
            "" => Command::Unknown(""),
            _ => Command::Unknown("unknown"),
        }
    }
}

// Shell结构
pub struct Shell {
    running: bool,
}

impl Shell {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn run(&mut self) {
        uart::println("Rust OS Shell v1.0");
        uart::println("Type 'help' for available commands");

        while self.running {
            uart::print("> ");

            // 读取用户输入
            let input = self.read_input();

            // 解析并执行命令
            self.execute_command(&input);
        }
    }

    fn read_input(&self) -> &'static str {
        // 简化实现，返回固定输入
        "help"
    }

    fn execute_command(&mut self, input: &str) {
        let command = Command::parse(input);

        match command {
            Command::Help => {
                uart::println("Available commands:");
                uart::println("  help   - Show this help message");
                uart::println("  list   - List all processes");
                uart::println("  create - Create a new process");
                uart::println("  exit   - Exit the shell");
            }
            Command::List => {
                uart::println("Process list:");
                process::list_processes();
            }
            Command::Create => {
                uart::println("Creating new process...");
                if let Some(pid) = process::create_process(4096) {
                    uart::print("Process created with PID: ");
                    uart::println(&pid.to_string());
                } else {
                    uart::println("Failed to create process");
                }
            }
            Command::Exit => {
                uart::println("Goodbye!");
                self.running = false;
            }
            Command::Unknown(cmd) => {
                if !cmd.is_empty() {
                    uart::print("Unknown command: ");
                    uart::println(&cmd);
                    uart::println("Type 'help' for available commands");
                }
            }
        }
    }
}

// 全局Shell实例
pub static mut SHELL: Shell = Shell { running: true };

// 全局函数
pub fn run_shell() {
    unsafe {
        SHELL.run();
    }
}
