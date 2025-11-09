/// Requires Console permission
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        unsafe {
            $crate::bindings::log(s.as_ptr(), s.len() as i32);
        }
    }};
}
