use crate::test_case::TestCase;
use crate::test_suite::TestSuite;

use crate::request::build;
use crate::response::{SearchResponse, SearchResults};
use crate::test_result::TestResult;
use anyhow::{Context, Result};

trait SearchEngine {
    fn search(&self, test_case: &TestCase) -> Result<SearchResponse>;
}

#[derive(Debug)]
pub struct LiveSearchEngine {
    endpoint: String,
}

impl LiveSearchEngine {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

impl SearchEngine for LiveSearchEngine {
    fn search(&self, test_case: &TestCase) -> Result<SearchResponse> {
        let response = build(self.endpoint.clone(), test_case).send()?;
        response.json().context("Could not get JSON from response")
    }
}

#[derive(Debug)]
pub struct Runner<T: SearchEngine> {
    search_engine: T,
    test_cases: Vec<TestCase>,
}

impl<T: SearchEngine> Runner<T> {
    pub fn new(suite: TestSuite, search_engine: T) -> Result<Self> {
        let test_cases = suite.test_cases().collect::<Result<Vec<_>>>()?;

        Ok(Self {
            search_engine,
            test_cases,
        })
    }

    pub fn run(&self) -> impl Iterator<Item = TestResult> {
        self.test_cases
            .iter()
            .map(|test_case| self.run_test(test_case))
    }

    fn run_test(&self, test_case: &TestCase) -> TestResult {
        let response = self.search_engine.search(test_case);
        let results = Self::search_results(response);
        TestResult::new(test_case, results)
    }

    fn search_results(response: Result<SearchResponse>) -> Result<SearchResults> {
        match response {
            Ok(response) => Ok(SearchResults::from(response)),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FakeSearchEngine;

    impl SearchEngine for FakeSearchEngine {
        fn search(&self, _test_case: &TestCase) -> Result<SearchResponse> {
            todo!()
        }
    }

    fn suite_with_test_case() -> TestSuite {
        TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
            selected-languages = ["pli", "en"]
            match-partial = false
            limit = 50
            site-language = "en"
            restrict = "all"
            "#,
        )
        .unwrap()
    }

    #[test]
    fn all_good_test_cases_gives_new_runner() {
        let suite = suite_with_test_case();
        let runner = Runner::new(suite, FakeSearchEngine {}).unwrap();
        assert_eq!(runner.test_cases.len(), 1)
    }

    fn suite_with_bad_test_case() -> TestSuite {
        TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
            "#,
        )
        .unwrap()
    }

    #[test]
    fn bad_test_fails_to_give_a_runner() {
        let suite = suite_with_bad_test_case();
        let error = Runner::new(suite, FakeSearchEngine {}).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Test case `Search for the metta sutta in English and Pali` missing `site-language` and no default provided."
        );
    }
}
