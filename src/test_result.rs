use crate::outcome::Outcome;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use crate::timed_search_results::TimedSearchResults;
use std::time::Duration;
// use crate::response::search_results::SearchResults;

#[derive(Clone, Debug, PartialEq)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
    pub outcome: Outcome,
}

impl TestResult {
    pub fn new(test_case: &TestCase, response: TimedResponse) -> Self {
        // let search_type = test_case.search_type();
        // let json = response.json;
        // let search_results = SearchResults::new(search_type, json);

        let timed_search_results = TimedSearchResults::from(response);

        Self {
            description: test_case.description.clone(),
            elapsed: timed_search_results.elapsed,
            outcome: Outcome::new(&test_case.expected, &timed_search_results.results),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_case::TestCase;
    use anyhow::anyhow;
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

    const JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : [ { "uid": "mn1" } ]
    }
    "#;

    fn ok_response() -> TimedResponse {
        TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Ok(String::from(JSON)),
        }
    }

    #[test]
    fn when_search_results_is_error_outcome_is_error() {
        let response = TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Err(anyhow!("Something went wrong")),
        };

        let test_result = TestResult::new(&test_case(), response);

        assert_eq!(
            test_result.outcome,
            Outcome::Error {
                message: String::from("Something went wrong")
            },
        );
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
