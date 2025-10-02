use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use std::time::Duration;

#[derive(Debug)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
}

impl TestResult {
    pub fn new(test_case: &TestCase, timed: &TimedSearchResults) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed: timed.elapsed,
        }
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
    use crate::identifiers::SuttaplexUid;
    use crate::response::SearchResults;
    use crate::search_service::TimedSearchResults;
    use crate::test_case::TestCase;
    use crate::test_result::{CategorySearch, TestResult};
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
}
