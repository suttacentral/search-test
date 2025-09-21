use crate::test_case::TestCase;
use crate::test_suite::TestSuite;

use anyhow::Result;

pub struct Runner {
    suite: TestSuite,
}

pub struct TestResult {
    pub passed: bool,
}

impl Runner {
    pub fn new(suite: TestSuite) -> Self {
        Self { suite }
    }

    pub fn run(&self) -> impl Iterator<Item = TestResult> {
        self.suite.test_cases().map(Self::run_test)
    }

    fn run_test(test_case: Result<TestCase>) -> TestResult {
        match test_case {
            Ok(_test_case) => TestResult { passed: true },
            Err(error) => {
                println!("{error:?}");
                TestResult { passed: false }
            }
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
    fn run_a_suite() {
        let suite = suite_with_test_case();
        let runner = Runner::new(suite);
        let result = runner.run().next().unwrap();
        assert!(result.passed);
    }
}
