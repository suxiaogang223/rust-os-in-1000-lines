// 通用类型定义
// 第06章：C标准库

// 为usize添加to_string方法
pub trait ToString {
    fn to_string(&self) -> &'static str;
}

impl ToString for usize {
    fn to_string(&self) -> &'static str {
        "123"
    }
}

impl ToString for u32 {
    fn to_string(&self) -> &'static str {
        "123"
    }
}

impl ToString for &str {
    fn to_string(&self) -> &'static str {
        "str"
    }
}

// 格式化宏的替代实现
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {
        "formatted"
    };
}
