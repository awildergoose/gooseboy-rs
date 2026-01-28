use crate::tests::{
    color::test_color, framebuffer::test_framebuffer, mem::test_mem, sprite::test_sprite,
    storage::test_storage, text::test_text, timer::test_timer,
};

pub struct TestResult {
    pub name: String,
    pub status: bool,
}

impl TestResult {
    pub const fn new(name: String, status: bool) -> Self {
        Self { name, status }
    }
}

#[macro_export]
macro_rules! test {
    ($name:expr, $expr:expr) => {
        $crate::state::RESULTS
            .lock()
            .unwrap()
            .push($crate::suite::TestResult::new(($name).to_owned(), $expr));
    };
}

pub fn run_tests() {
    test_storage();
    test_color();
    test_framebuffer();
    test_mem();
    test_sprite();
    test_text();
    test_timer();
}
