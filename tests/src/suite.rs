use crate::tests::storage::test_storage;

pub struct TestResult {
    pub name: String,
    pub status: bool,
}

impl TestResult {
    pub fn new(name: String, status: bool) -> Self {
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
}
