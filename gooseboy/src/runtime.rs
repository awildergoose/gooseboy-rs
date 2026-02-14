use crate::unsafe_casts;

/// Requires Console permission
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        $crate::runtime::log_str(&s);
    }};
}

pub fn log_str(s: &str) {
    unsafe {
        let len = unsafe_casts::str_len(s);
        crate::bindings::log(s.as_ptr(), len);
    }
}
