use crate::outcome::Outcome;
use crate::rank::Rank;
use crate::search_results::SearchResults;
use crate::summary::Summary;
use crate::test_result::TestResult;
use std::fmt::{Display, Formatter};

impl TestResult {
    fn main_line(&self) -> String {
        let summary = Summary::from(&self.outcome).to_string();
        let elapsed = format!("{}ms", self.elapsed.as_millis());
        let description = &self.description;
        format!("{summary:7} {elapsed:6} {description}")
    }

    fn detail_line(&self) -> Option<String> {
        match &self.outcome {
            Outcome::Error { message } => Some(message.to_string()),
            Outcome::Success => None,
            Outcome::Found { results: _ } => None,
            Outcome::NotFound { results } => Some(Self::not_found_message(results)),
            Outcome::Ranked { results, rank } => match rank {
                Rank::NotFound { minimum } => Some(Self::rank_not_found_message(results, minimum)),
                Rank::TooLow { minimum, actual } => {
                    Some(Self::rank_too_low_message(results, minimum, actual))
                }
                Rank::Sufficient {
                    minimum: _,
                    actual: _,
                } => None,
            },
        }
    }

    fn search_term(results: &SearchResults) -> String {
        match results {
            SearchResults::Text {
                expected,
                results: _,
            } => format!("Text hit {expected}"),
            SearchResults::Dictionary {
                expected,
                results: _,
            } => format!("Dictionary hit {expected}"),
            SearchResults::Suttaplex {
                expected,
                results: _,
            } => format!("Suttaplex hit {expected}"),
            SearchResults::Volpage {
                expected,
                results: _,
            } => todo!(),
        }
    }

    fn not_found_message(results: &SearchResults) -> String {
        format!("{} not found in search results", Self::search_term(results))
    }

    fn rank_not_found_message(results: &SearchResults, minimum: &usize) -> String {
        format!(
            "Minium rank {minimum} expected for {} but it was not found",
            Self::search_term(results)
        )
    }

    fn rank_too_low_message(results: &SearchResults, minimum: &usize, actual: &usize) -> String {
        format!(
            "Expected {} to have minimum rank of {minimum} but it was found at rank {actual}",
            Self::search_term(results)
        )
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.main_line())?;
        if let Some(detail_line) = self.detail_line() {
            writeln!(f, "  {}", detail_line)?;
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
    use crate::identifiers::SuttaplexUid;
    use std::io::Write;
    use std::time::Duration;

    #[test]
    fn display_summary() {
        assert_eq!(Summary::Error.to_string(), "ERROR");
        assert_eq!(Summary::Failed.to_string(), "FAILED");
        assert_eq!(Summary::Passed.to_string(), "PASSED");
    }

    fn message(line_1: &str, line_2: Option<&str>) -> String {
        let mut expected = Vec::new();
        writeln!(&mut expected, "{line_1}").unwrap();
        if let Some(line_2) = line_2 {
            writeln!(&mut expected, "{line_2}").unwrap();
        }
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
                Some("  Something went wrong"),
            )
        );
    }

    #[test]
    fn display_success() {
        let test_result = TestResult {
            description: String::from("We will retrieve something"),
            elapsed: Duration::from_millis(321),
            outcome: Outcome::Success,
        };

        assert_eq!(
            test_result.to_string(),
            message("PASSED  321ms  We will retrieve something", None)
        );
    }

    #[test]
    fn display_found() {
        let test_result = TestResult {
            description: String::from("Find suttaplex mn1"),
            elapsed: Duration::from_millis(21),
            outcome: Outcome::Found {
                results: SearchResults::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
                },
            },
        };

        assert_eq!(
            test_result.to_string(),
            message("PASSED  21ms   Find suttaplex mn1", None,)
        );
    }

    #[test]
    fn display_not_found() {
        let test_result = TestResult {
            description: String::from("Find suttaplex mn1"),
            elapsed: Duration::from_millis(1),
            outcome: Outcome::NotFound {
                results: SearchResults::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    results: vec![],
                },
            },
        };

        assert_eq!(
            test_result.to_string(),
            message(
                "FAILED  1ms    Find suttaplex mn1",
                Some("  Suttaplex hit mn1 not found in search results")
            )
        )
    }

    #[test]
    fn display_ranked_not_found() {
        let test_result = TestResult {
            description: String::from("Wanted rank, but not found"),
            elapsed: Duration::from_millis(10),
            outcome: Outcome::Ranked {
                results: SearchResults::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    results: vec![],
                },
                rank: Rank::NotFound { minimum: 3 },
            },
        };

        assert_eq!(
            test_result.to_string(),
            message(
                "FAILED  10ms   Wanted rank, but not found",
                Some("  Minium rank 3 expected for Suttaplex hit mn1 but it was not found")
            )
        )
    }

    #[test]
    fn display_ranked_too_low() {
        let test_result = TestResult {
            description: String::from("Expecting top rank"),
            elapsed: Duration::from_millis(76),
            outcome: Outcome::Ranked {
                results: SearchResults::Suttaplex {
                    expected: SuttaplexUid::from("mn2"),
                    results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
                },
                rank: Rank::TooLow {
                    minimum: 1,
                    actual: 2,
                },
            },
        };

        assert_eq!(
            test_result.to_string(),
            message(
                "FAILED  76ms   Expecting top rank",
                Some(
                    "  Expected Suttaplex hit mn2 to have minimum rank of 1 but it was found at rank 2"
                )
            )
        );
    }

    #[test]
    fn display_ranked_sufficient() {
        let test_result = TestResult {
            description: String::from("Expecting top rank"),
            elapsed: Duration::from_millis(123),
            outcome: Outcome::Ranked {
                results: SearchResults::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
                },
                rank: Rank::Sufficient {
                    minimum: 1,
                    actual: 1,
                },
            },
        };

        assert_eq!(
            test_result.to_string(),
            message("PASSED  123ms  Expecting top rank", None)
        );
    }
}
