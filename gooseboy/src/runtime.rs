//! Used to hold runtime (or just util) functions.
use crate::unsafe_casts;

/// Logs text to the console.
/// Requires [`Console`](crate::system::Permission::Console) permission
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        $crate::runtime::log_str(&s);
    }};
}

/// Logs text to the console.
/// Requires [`Console`](crate::system::Permission::Console) permission
pub fn log_str(s: &str) {
    unsafe {
        let len = unsafe_casts::str_len(s);
        crate::bindings::log(s.as_ptr(), len);
    }
}
