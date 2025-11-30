use crate::summary::Summary;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct ResultCount {
    passed: usize,
    failed: usize,
    error: usize,
}

impl ResultCount {
    pub fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            error: 0,
        }
    }

    pub fn add(&mut self, summary: &Summary) {
        match summary {
            Summary::Passed => self.passed += 1,
            Summary::Failed => self.failed += 1,
            Summary::Error => self.error += 1,
        }
    }
}

impl Display for ResultCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} passed, {} failed, {} encountered an error",
            self.passed, self.failed, self.error
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialise_result_count() {
        let counter = ResultCount::new();
        assert_eq!(
            counter,
            ResultCount {
                passed: 0,
                failed: 0,
                error: 0
            }
        );
    }

    #[test]
    fn add_one_of_each() {
        let mut counter = ResultCount::new();
        counter.add(&Summary::Passed);
        counter.add(&Summary::Failed);
        counter.add(&Summary::Error);

        assert_eq!(
            counter,
            ResultCount {
                passed: 1,
                failed: 1,
                error: 1,
            }
        );
    }

    #[test]
    fn display() {
        let result_count = ResultCount {
            passed: 1,
            failed: 2,
            error: 0,
        };

        assert_eq!(
            result_count.to_string(),
            "1 passed, 2 failed, 0 encountered an error"
        )
    }
}
