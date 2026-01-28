/// Requires Console permission
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        unsafe {
            $crate::bindings::log(s.as_ptr(), $crate::unsafe_casts::str_len(s));
        }
    }};
}
