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
        RESULTS
            .lock()
            .unwrap()
            .push(TestResult::new(($name).to_owned(), $expr));
    };
}
