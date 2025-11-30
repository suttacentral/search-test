use crate::test_result::{Outcome, Rank};

#[derive(Clone, Debug, PartialEq)]
pub enum Summary {
    Error,
    Passed,
    Failed,
}

impl From<&Outcome> for Summary {
    fn from(outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Error { message: _ } => Summary::Error,
            Outcome::Success => Summary::Passed,
            Outcome::Found { search: _ } => Summary::Passed,
            Outcome::NotFound { search: _ } => Summary::Failed,
            Outcome::Ranked { search: _, rank } => match rank {
                Rank::NotFound { minimum: _ } => Summary::Failed,
                Rank::TooLow {
                    minimum: _,
                    actual: _,
                } => Summary::Failed,
                Rank::Sufficient {
                    minimum: _,
                    actual: _,
                } => Summary::Passed,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
