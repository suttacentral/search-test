use crate::test_result::TestResult;
use std::fmt::{Display, Formatter};

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ran test {} in {}ms",
            self.description,
            self.elapsed.as_millis()
        )
    }
}
