/// Requires Console permission
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        $crate::runtime::log_str(&s);
    }};
}

pub fn log_str(s: &str) {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    unsafe {
        let len = s.len() as i32;
        crate::bindings::log(s.as_ptr(), len);
    }
}
