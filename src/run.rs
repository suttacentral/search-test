use crate::test_case::TestCase;
use crate::test_suite::TestSuite;

use crate::search_service::SearchService;
use crate::test_result::TestResult;
use anyhow::Result;

#[derive(Debug)]
pub struct Runner<T: SearchService> {
    search_service: T,
    test_cases: Vec<TestCase>,
}

impl<T: SearchService> Runner<T> {
    pub fn new(suite: TestSuite, search_service: T) -> Result<Self> {
        let test_cases = suite.test_cases().collect::<Result<Vec<_>>>()?;

        Ok(Self {
            search_service,
            test_cases,
        })
    }

    pub fn run(&self) -> impl Iterator<Item = TestResult> {
        self.test_cases
            .iter()
            .map(|test_case| self.run_test(test_case))
    }

    fn run_test(&self, test_case: &TestCase) -> TestResult {
        let results = self.search_service.search(test_case);
        TestResult::new(test_case, &results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::SuttaplexUid;
    use crate::response::SearchResults;
    use crate::search_service::TimedSearchResults;
    use std::cell::RefCell;
    use std::time::Duration;

    #[derive(Debug)]
    struct FakeSearchService {
        results: RefCell<Vec<TimedSearchResults>>,
    }

    impl FakeSearchService {
        fn new(results: Vec<TimedSearchResults>) -> FakeSearchService {
            Self {
                results: RefCell::new(results),
            }
        }
    }

    impl SearchService for FakeSearchService {
        fn search(&self, _: &TestCase) -> TimedSearchResults {
            self.results.borrow_mut().pop().unwrap()
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
        let runner = Runner::new(suite, FakeSearchService::new(Vec::new())).unwrap();
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
        let error = Runner::new(suite, FakeSearchService::new(Vec::new())).unwrap_err();
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

        let search_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: vec![SuttaplexUid::from("snp1.8")],
            }),
        };

        let engine = FakeSearchService::new(vec![search_results]);
        let runner = Runner::new(suite, engine).unwrap();
        let test_result = runner.run().next().unwrap();
        assert!(test_result.passed);
        assert_eq!(test_result.elapsed, Duration::from_secs(3))
    }
}
