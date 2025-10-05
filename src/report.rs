use crate::test_result::{Summary, TestResult};
use std::fmt::{Display, Formatter};

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Summary::Error => f.write_str("ERROR"),
            Summary::Failed => f.write_str("FAILED"),
            Summary::Passed => f.write_str("PASSED"),
        }
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: Ran test {} in {}ms",
            self.outcome.summary(),
            self.description,
            self.elapsed.as_millis()
        )
    }
}
