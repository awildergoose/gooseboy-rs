#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        unsafe {
            $crate::bindings::log(s.as_ptr() as i32, s.len() as i32);
        }
    }};
}
