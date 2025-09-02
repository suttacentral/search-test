pub mod results;
pub mod suite;

use crate::results::SearchResults;
use crate::suite::TestSuite;
use anyhow::{Context, Result};
use reqwest::Error;
use reqwest::blocking::{Client, RequestBuilder};
use std::fmt;
use std::fmt::Display;

fn test_suite() -> TestSuite {
    TestSuite::load_from_string(
        r#"
        [settings]
        endpoint = "http://localhost/api/search/instant"

        [defaults]
        limit = 1
        site-language = "en"
        restrict = "all"
        match-partial=false
        selected-languages = ["en", "pli"]

        [[test-case]]
        description = "The Simile of the Adze"
        query = "adze"
        "#,
    )
    .unwrap()
}

fn build_request(endpoint: String, test_case: suite::TestCase) -> RequestBuilder {
    let params = vec![
        ("limit", test_case.limit.to_string()),
        ("query", test_case.query),
        ("language", test_case.site_language),
        ("restrict", test_case.restrict),
        ("matchpartial", test_case.match_partial.to_string()),
    ];

    Client::new()
        .post(endpoint.as_str())
        .query(&params)
        .json(&test_case.selected_languages)
}

impl Display for SearchResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} results", self.total)?;
        writeln!(f, "{} hits", self.hits.len())?;
        for hit in &self.hits {
            writeln!(f, "{hit}")?;
        }
        for suttaplex in &self.suttaplex {
            writeln!(f, "Suttaplex result: {}", suttaplex.uid)?;
        }
        for fuzzy in &self.fuzzy_dictionary {
            writeln!(f, "Fuzzy dictionary result: {}", fuzzy.url)?;
        }
        Ok(())
    }
}

fn run_tests(toml: &str) -> Result<SearchResults> {
    let suite = TestSuite::load_from_string(toml)?;
    let test_cases = suite.test_cases()?;
    todo!()
}

fn main() {
    let suite = test_suite();
    let test_cases = suite.test_cases().unwrap();

    for test_case in test_cases {
        let request = build_request(suite.endpoint(), test_case);
        let response = request.send().unwrap();
        let results: Result<SearchResults, Error> = response.json();

        match results {
            Ok(parsed_results) => println!("{parsed_results}"),
            Err(error) => {
                println!("An error occurred parsing response.");
                println!("{error:?}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_correct_url() {
        let suite = test_suite();
        let test_case = suite.test_cases().unwrap().iter().next().unwrap().clone();
        let request = build_request(suite.endpoint(), test_case).build().unwrap();
        let expected = "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false";
        let actual = request.url().to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn has_correct_body() {
        let suite = test_suite();
        let test_case = suite.test_cases().unwrap().iter().next().unwrap().clone();
        let request = build_request(suite.endpoint(), test_case).build().unwrap();
        let body = request.body().unwrap().as_bytes().unwrap();
        let body_contents = str::from_utf8(body).unwrap().to_string();
        assert_eq!(body_contents, "[\"en\",\"pli\"]");
    }

    #[test]
    fn running_tests_with_malformed_toml_gives_error() {
        let error = run_tests("This is not TOML").unwrap_err();
        assert_eq!(error.to_string(), "Failed to parse TOML.");
    }

    #[test]
    fn running_invalid_test_gives_error() {
        let no_limit = r#"
        [settings]
        endpoint = "http://localhost/api/search/instant"

        [defaults]
        site-language = "en"
        restrict = "all"
        match-partial=false
        selected-languages = ["en", "pli"]

        [[test-case]]
        description = "The Simile of the Adze"
        query = "adze"
        "#;

        let error = run_tests(no_limit).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Test case missing limit and no default provided."
        );
    }
}
