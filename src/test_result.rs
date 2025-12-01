use crate::outcome::Outcome;
use crate::response::search_results::SearchResultsNewStyle;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use crate::timed_search_results::TimedSearchResults;
use anyhow::Result;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
    pub outcome: Outcome,
}

impl TestResult {
    pub fn new(test_case: &TestCase, response: TimedResponse) -> Self {
        let timed_search_results = TimedSearchResults::from(response);

        Self {
            description: test_case.description.clone(),
            elapsed: timed_search_results.elapsed,
            outcome: Outcome::new(&test_case.expected, &timed_search_results.results),
        }
    }

    fn new_style_results(
        test_case: &TestCase,
        json: Result<String>,
    ) -> Result<SearchResultsNewStyle> {
        let json = json?;
        match test_case.search_type() {
            None => todo!(),
            Some(search_type) => SearchResultsNewStyle::new(search_type, json.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected::Expected;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
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

    #[test]
    fn new_style_results_when_no_json() {
        let results =
            TestResult::new_style_results(&test_case(), Err(anyhow!("Failed to get JSON")))
                .unwrap_err();
        assert_eq!(results.to_string(), "Failed to get JSON")
    }

    #[test]
    fn new_style_results_when_bad_json() {
        let test_case = TestCase {
            expected: Some(Expected::Unranked {
                key: SearchResultKey::Suttaplex {
                    uid: SuttaplexUid::from("mn1"),
                },
            }),
            ..test_case()
        };

        let results =
            TestResult::new_style_results(&test_case, Ok(String::from("This is not JSON")))
                .unwrap_err();
        assert_eq!(results.to_string(), "expected value at line 1 column 1") // TODO: Improve error message.
    }

    fn ok_response() -> TimedResponse {
        TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Ok(String::from(JSON)),
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
