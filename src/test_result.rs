use crate::outcome::Outcome;
use crate::response::search_results::SearchResults;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
    pub outcome: Outcome,
}

impl TestResult {
    pub fn new(test_case: &TestCase, response: TimedResponse) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed: response.elapsed,
            outcome: Outcome::new_with_new_style_results(&test_case.expected, response.json),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_case::TestCase;
    use crate::test_json::SUTTAPLEX_MN1_JSON;
    use std::time::Duration;

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

    fn ok_response() -> TimedResponse {
        TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Ok(String::from(SUTTAPLEX_MN1_JSON)),
        }
    }

    #[test]
    fn test_result_has_description() {
        let test_case = TestCase {
            description: "Matching description".to_string(),
            ..test_case()
        };

        let test_result = TestResult::new(&test_case, ok_response());
        assert_eq!(test_result.description, "Matching description");
    }

    #[test]
    fn test_result_has_elapsed_time() {
        let test_result = TestResult::new(&test_case(), ok_response());
        assert_eq!(test_result.elapsed, Duration::from_secs(3));
    }
}
