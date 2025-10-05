use crate::test_result::{Summary, TestResult};
use std::fmt::{Display, Formatter};

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.main_line())
    }
}

impl TestResult {
    fn main_line(&self) -> String {
        let summary = self.outcome.summary().to_string();
        let elapsed = format!("{}ms", self.elapsed.as_millis());
        let description = &self.description;

        format!("{summary:7} {elapsed:6} {description}")
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Summary::Error => f.write_str("ERROR"),
            Summary::Failed => f.write_str("FAILED"),
            Summary::Passed => f.write_str("PASSED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_result::Outcome;
    use std::time::Duration;

    #[test]
    fn display_summary() {
        assert_eq!(Summary::Error.to_string(), "ERROR");
        assert_eq!(Summary::Failed.to_string(), "FAILED");
        assert_eq!(Summary::Passed.to_string(), "PASSED");
    }

    #[test]
    fn display_error_main_line() {
        let test_result = TestResult {
            description: String::from("Something will go wrong."),
            elapsed: Duration::from_millis(4321),
            outcome: Outcome::Error {
                message: String::from("Something went wrong"),
            },
        };
        assert_eq!(
            test_result.main_line(),
            "ERROR   4321ms Something will go wrong."
        )
    }
}
