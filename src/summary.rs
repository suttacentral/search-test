use crate::outcome::Outcome;
use crate::rank::Rank;

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
            Outcome::Found { results: _ } => Summary::Passed,
            Outcome::NotFound { results: _ } => Summary::Failed,
            Outcome::Ranked { results: _, rank } => match rank {
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
    use crate::identifiers::SuttaplexUid;
    use crate::response::search_results::SearchResults;

    #[test]
    fn summary_error_is_error() {
        let outcome = Outcome::Error {
            message: String::from("An error occured"),
        };
        assert_eq!(Summary::from(&outcome), Summary::Error);
    }

    #[test]
    fn summary_is_passed_for_success() {
        let outcome = Outcome::Success;
        assert_eq!(Summary::from(&outcome), Summary::Passed);
    }

    #[test]
    fn summary_is_passed_for_found() {
        let outcome = Outcome::Found {
            results: SearchResults::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                results: Vec::new(),
            },
        };
        assert_eq!(Summary::from(&outcome), Summary::Passed);
    }

    #[test]
    fn summary_is_failed_for_not_found() {
        let outcome = Outcome::NotFound {
            results: SearchResults::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                results: Vec::new(),
            },
        };
        assert_eq!(Summary::from(&outcome), Summary::Failed);
    }

    #[test]
    fn summary_is_failed_for_rank_not_found() {
        let outcome = Outcome::Ranked {
            results: SearchResults::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                results: Vec::new(),
            },
            rank: Rank::NotFound { minimum: 3 },
        };

        assert_eq!(Summary::from(&outcome), Summary::Failed);
    }

    #[test]
    fn summary_is_failed_for_rank_too_low() {
        let outcome = Outcome::Ranked {
            results: SearchResults::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                results: Vec::new(),
            },
            rank: Rank::TooLow {
                minimum: 3,
                actual: 4,
            },
        };

        assert_eq!(Summary::from(&outcome), Summary::Failed);
    }

    #[test]
    fn summary_is_passed_for_rank_sufficient() {
        let outcome = Outcome::Ranked {
            results: SearchResults::Suttaplex {
                expected: SuttaplexUid::from("mn1"),
                results: Vec::new(),
            },
            rank: Rank::Sufficient {
                minimum: 3,
                actual: 2,
            },
        };

        assert_eq!(Summary::from(&outcome), Summary::Passed);
    }
}
