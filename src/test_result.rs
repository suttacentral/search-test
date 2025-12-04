use crate::expected::Expected;
use crate::outcome::Outcome;
use crate::response::search_results::SearchResultsNewStyle;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use crate::timed_search_results::TimedSearchResults;
use anyhow::{Context, Result};
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
            outcome: Outcome::new_with_old_style_results(
                &test_case.expected,
                &timed_search_results.results,
            ),
        }
    }

    fn new_style_results(
        expected: &Option<Expected>,
        json: Result<String>,
    ) -> Result<Option<SearchResultsNewStyle>> {
        // We choose the parser based on what is expected. If we don't expect anything then we
        // can't choose a parser. Therefore, if expected is None, we don't parse the JSON
        // and won't know if it is well-formed so we just return Ok(None)
        let json = json?;
        match expected {
            None => Ok(None),
            Some(expected) => {
                let results = SearchResultsNewStyle::new(expected.search_type(), json.as_str())
                    .context("Could not extract search results from server response");
                match results {
                    Ok(results) => Ok(Some(results)),
                    Err(error) => Err(error),
                }
            }
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

    const GOOD_JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : [ { "uid": "mn1" } ]
    }
    "#;

    const BAD_JSON: &str = "This is not JSON";

    #[test]
    fn new_style_results_when_error_getting_json() {
        let results = TestResult::new_style_results(
            &test_case().expected,
            Err(anyhow!("Failed to get JSON")),
        )
        .unwrap_err();
        assert_eq!(results.to_string(), "Failed to get JSON")
    }

    #[test]
    fn new_style_results_when_something_expected_and_json_is_bad() {
        let expected = Some(Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        });

        let results =
            TestResult::new_style_results(&expected, Ok(String::from(BAD_JSON))).unwrap_err();
        assert_eq!(
            results.to_string(),
            "Could not extract search results from server response"
        )
    }

    #[test]
    fn new_style_results_when_nothing_is_expected_and_json_is_bad() {
        assert!(
            TestResult::new_style_results(&None, Ok(String::from(BAD_JSON)))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn new_style_results_when_nothing_expected_and_json_is_good() {
        assert!(
            TestResult::new_style_results(&None, Ok(String::from(GOOD_JSON)))
                .unwrap()
                .is_none()
        )
    }

    #[test]
    fn new_style_results_when_something_expected_and_json_is_good() {
        let expected = Some(Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        });

        assert_eq!(
            TestResult::new_style_results(&expected, Ok(String::from(GOOD_JSON))).unwrap(),
            Some(SearchResultsNewStyle::Suttaplex {
                results: vec![SuttaplexUid::from("mn1")]
            })
        )
    }

    fn ok_response() -> TimedResponse {
        TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Ok(String::from(GOOD_JSON)),
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
