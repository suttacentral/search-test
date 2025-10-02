use crate::expected::Expected;
use crate::response::SearchResults;
use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use anyhow::{Result, anyhow};
use std::time::Duration;

#[derive(Debug)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
    pub outcome: Result<Outcome>,
}

impl TestResult {
    pub fn new(test_case: &TestCase, timed: &TimedSearchResults) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed: timed.elapsed,
            outcome: Self::outcome(test_case, timed),
        }
    }

    fn outcome(test_case: &TestCase, timed: &TimedSearchResults) -> Result<Outcome> {
        match &timed.results {
            Ok(search_results) => Ok(Outcome::new(&test_case.expected, search_results)),
            Err(error) => Err(anyhow!(error.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Success,
    Found,
    NotFound,
}

impl Outcome {
    fn new(expected: &Option<Expected>, search_results: &SearchResults) -> Self {
        match expected {
            None => Self::Success,
            Some(exp) => Self::found(),
        }
    }

    fn found() -> Self {
        Self::Found
    }
}

struct CategorySearch<C>
where
    C: PartialEq,
{
    id: C,
    sequence: Vec<C>,
}

impl<C: PartialEq> CategorySearch<C> {
    fn found(&self) -> bool {
        self.sequence.contains(&self.id)
    }

    fn rank(&self) -> Option<usize> {
        self.sequence
            .iter()
            .position(|hit| hit == &self.id)
            .map(|position| position + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
    use crate::response::SearchResults;
    use crate::search_service::TimedSearchResults;
    use crate::test_case::TestCase;
    use anyhow::anyhow;
    use std::time::Duration;

    #[test]
    fn suttaplex_found() {
        let search = CategorySearch {
            id: SuttaplexUid::from("mn1"),
            sequence: vec![SuttaplexUid::from("mn1")],
        };

        assert!(search.found());
    }

    #[test]
    fn suttaplex_ranked() {
        let search = CategorySearch {
            id: SuttaplexUid::from("mn2"),
            sequence: vec![
                SuttaplexUid::from("mn1"),
                SuttaplexUid::from("mn2"),
                SuttaplexUid::from("mn3"),
            ],
        };

        assert_eq!(search.rank(), Some(2));
    }

    fn test_case() -> TestCase {
        TestCase {
            description: "Description".to_string(),
            query: "query".to_string(),
            site_language: "en".to_string(),
            selected_languages: vec!["en".to_string()],
            match_partial: false,
            limit: 50,
            restrict: "all".to_string(),
            expected: None,
        }
    }

    fn search_results() -> TimedSearchResults {
        TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: Vec::new(),
            }),
        }
    }

    #[test]
    fn test_result_has_description() {
        let test_case = TestCase {
            description: "Matching description".to_string(),
            ..test_case()
        };

        let test_result = TestResult::new(&test_case, &search_results());
        assert_eq!(test_result.description, "Matching description");
    }

    #[test]
    fn test_result_has_elapsed_time() {
        let search_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            ..search_results()
        };
        let test_result = TestResult::new(&test_case(), &search_results);
        assert_eq!(test_result.elapsed, Duration::from_secs(3));
    }

    #[test]
    fn when_search_results_is_error_outcome_is_error() {
        let search_results = TimedSearchResults {
            results: Err(anyhow!("Something went wrong")),
            ..search_results()
        };

        let test_result = TestResult::new(&test_case(), &search_results);

        assert_eq!(
            test_result.outcome.unwrap_err().to_string(),
            "Something went wrong"
        );
    }

    #[test]
    fn new_outcome_is_success_when_nothing_expected() {
        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        let outcome = Outcome::new(&None, &search_results);
        assert_eq!(outcome, Outcome::Success);
    }

    #[test]
    fn outcome_is_found_in_search() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("mn1")],
        };

        let outcome = Outcome::new(&Some(expected), &search_results);

        assert_eq!(outcome, Outcome::Found);
    }
}
