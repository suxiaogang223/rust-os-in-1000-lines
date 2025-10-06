// 字符串处理函数
// 第06章：C标准库

// 简单的字符串长度计算
pub fn strlen(s: &str) -> usize {
    s.len()
}

// 字符串比较
pub fn strcmp(s1: &str, s2: &str) -> i32 {
    if s1 == s2 {
        0
    } else {
        1
    }
}

// 字符串复制
pub unsafe fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dst.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    dst
}
