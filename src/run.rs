use crate::test_case::TestCase;
use crate::test_suite::TestSuite;

use crate::request::build;
use crate::response::{SearchResponse, SearchResults};
use crate::test_result::TestResult;
use anyhow::{Context, Result};

pub trait SearchEngine {
    fn search(&self, test_case: &TestCase) -> Result<SearchResults>;
}

#[derive(Debug)]
pub struct LiveSearchEngine {
    endpoint: String,
}

impl LiveSearchEngine {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    fn search_results(response: Result<SearchResponse>) -> Result<SearchResults> {
        match response {
            Ok(response) => Ok(SearchResults::from(response)),
            Err(error) => Err(error),
        }
    }
}

impl SearchEngine for LiveSearchEngine {
    fn search(&self, test_case: &TestCase) -> Result<SearchResults> {
        let http_response = build(self.endpoint.clone(), test_case).send()?;
        let search_response = http_response
            .json()
            .context("Could not get JSON from response");
        Self::search_results(search_response)
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
        let results = self.search_engine.search(test_case);
        TestResult::new(test_case, results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::SuttaplexUid;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct FakeSearchEngine {
        results: RefCell<Vec<SearchResults>>,
    }

    impl FakeSearchEngine {
        fn new(results: Vec<SearchResults>) -> FakeSearchEngine {
            Self {
                results: RefCell::new(results),
            }
        }
    }

    impl SearchEngine for FakeSearchEngine {
        fn search(&self, _test_case: &TestCase) -> Result<SearchResults> {
            self.results.borrow_mut().pop().context("No results left")
        }
    }

    #[test]
    fn good_test_gives_new_runner() {
        let suite = TestSuite::load_from_string(
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
        .unwrap();
        let runner = Runner::new(suite, FakeSearchEngine::new(Vec::new())).unwrap();
        assert_eq!(runner.test_cases.len(), 1)
    }

    #[test]
    fn bad_test_fails_to_give_a_runner() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "metta"
            "#,
        )
        .unwrap();
        let error = Runner::new(suite, FakeSearchEngine::new(Vec::new())).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Test case `Search for the metta sutta in English and Pali` missing `site-language` and no default provided."
        );
    }

    #[test]
    fn run_a_test() {
        let suite = TestSuite::load_from_string(
            r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [defaults]
            selected-languages = ["pli", "en"]
            match-partial = false
            limit = 50
            site-language = "en"
            restrict = "all"

            [[test-case]]
            description = "First search"
            query = "metta"
            "#,
        )
        .unwrap();

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("snp1.8")],
        };

        let engine = FakeSearchEngine::new(vec![search_results]);
        let runner = Runner::new(suite, engine).unwrap();
        let test_result = runner.run().next().unwrap();
        assert!(test_result.passed);
    }
}
