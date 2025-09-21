use crate::test_case::TestCase;
use crate::test_suite::TestSuite;

use crate::request::build;
use crate::response::SearchResponse;
use anyhow::{Context, Result};

pub struct Runner {
    suite: TestSuite,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestResult {
    pub passed: bool,
}

impl Runner {
    pub fn new(suite: TestSuite) -> Self {
        Self { suite }
    }

    pub fn run(&self) -> impl Iterator<Item = TestResult> {
        self.suite
            .test_cases()
            .map(|test_case| self.run_test(test_case))
    }

    fn send(endpoint: String, test_case: TestCase) -> Result<SearchResponse> {
        let response = build(endpoint, test_case).send()?;
        response.json().context("Could not get JSON from response")
    }

    fn run_test(&self, test_case: Result<TestCase>) -> TestResult {
        match test_case {
            Ok(test_case) => {
                let response = Self::send(self.suite.endpoint(), test_case);
                match response {
                    Ok(_response) => TestResult { passed: true },
                    Err(_error) => TestResult { passed: false },
                }
            }
            Err(_error) => TestResult { passed: false },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[ignore]
    fn run_a_suite() {
        let suite = suite_with_test_case();
        let runner = Runner::new(suite);
        let result = runner.run().next().unwrap();
        assert!(result.passed);
    }
}
