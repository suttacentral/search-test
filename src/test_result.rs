use crate::response::SearchResults;
use crate::test_case::TestCase;
use anyhow::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestResult {
    pub passed: bool,
}

impl TestResult {
    pub fn new(test_case: TestCase, search_results: Result<SearchResults>) -> Self {
        Self { passed: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_test_result() {
        let test_case = TestCase {
            description: "Search in English only.".to_string(),
            query: "metta".to_string(),
            site_language: "en".to_string(),
            selected_languages: vec!["en".to_string()],
            match_partial: false,
            limit: 50,
            restrict: "all".to_string(),
            expected: None,
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        let _test_result = TestResult::new(test_case, Ok(search_results));
    }
}
