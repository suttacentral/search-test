use crate::test_result::{Outcome, Summary, TestResult};
use std::fmt::{Display, Formatter};

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let summary = self.outcome.summary().to_string();
        let elapsed = format!("{}ms", self.elapsed.as_millis());
        let description = &self.description;

        writeln!(f, "{summary:7} {elapsed:6} {description}")?;
        match &self.outcome {
            Outcome::Error { message } => writeln!(f, "  {message}")?,
            _ => todo!(),
        }
        Ok(())
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
    use std::io::Write;
    use std::time::Duration;

    #[test]
    fn display_summary() {
        assert_eq!(Summary::Error.to_string(), "ERROR");
        assert_eq!(Summary::Failed.to_string(), "FAILED");
        assert_eq!(Summary::Passed.to_string(), "PASSED");
    }

    fn message(line_1: &str, line_2: &str) -> String {
        let mut expected = Vec::new();
        writeln!(&mut expected, "{line_1}").unwrap();
        writeln!(&mut expected, "{line_2}").unwrap();
        String::from_utf8(expected).unwrap()
    }

    #[test]
    fn display_error() {
        let test_result = TestResult {
            description: String::from("Something will go wrong"),
            elapsed: Duration::from_millis(4321),
            outcome: Outcome::Error {
                message: String::from("Something went wrong"),
            },
        };

        assert_eq!(
            test_result.to_string(),
            message(
                "ERROR   4321ms Something will go wrong",
                "  Something went wrong",
            )
        );
    }
}
