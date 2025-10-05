use crate::test_result::TestResult;
use std::fmt::{Display, Formatter};

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("The test result")
    }
}
