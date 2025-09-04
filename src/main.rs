pub mod act;
pub mod arrange;

use crate::act::{SearchResults, build_request};
use crate::arrange::TestSuite;
use anyhow::{Context, Result};
use reqwest::Error;
use std::fmt::Display;

fn main() {
    let toml = std::fs::read_to_string("test-cases/play.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();
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
    use crate::arrange::{Assertions, SuttaHitAssertion, TestCase};

    fn test_case() -> TestCase {
        let assertions = Assertions {
            sutta_hits: SuttaHitAssertion {
                top: String::from("/kp9/pli/ms"),
            },
        };

        TestCase {
            query: String::from("metta"),
            description: String::from("Get the metta"),
            limit: 1,
            site_language: String::from("en"),
            restrict: String::from("all"),
            selected_languages: vec![String::from("en")],
            match_partial: false,
            assertions: Some(assertions),
        }
    }

    #[test]
    fn can_assert_top_sutta_hit() {
        let test_case = test_case();
        let expected = test_case.assertions.unwrap().sutta_hits.top;
        assert_eq!(expected, "/kp9/pli/ms");
    }
}
