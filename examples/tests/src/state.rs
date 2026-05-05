use std::sync::{LazyLock, Mutex};

use crate::suite::TestResult;

pub static RESULTS: LazyLock<Mutex<Vec<TestResult>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static PAGE_INDEX: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0usize));
