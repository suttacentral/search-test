use crate::test_suite::TestSuite;

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
        self.suite
            .test_cases()
            .map(|test_case| TestResult { passed: false })
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
            limit = 50
            site-language = "en"
            restrict = "all"
            "#,
        )
        .unwrap()
    }

    fn run_a_suite() {
        let suite = suite_with_test_case();
        let runner = Runner::new(suite);
        let result = runner.run().next().unwrap();
        assert!(result.passed);
    }
}
