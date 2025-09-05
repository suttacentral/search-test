pub mod act;
pub mod arrange;
mod identifiers;

use crate::act::{SearchResponse, build_request};
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
        let results: Result<SearchResponse, Error> = response.json();

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
    use crate::act::SearchResponse;
    use crate::arrange::{Assertions, SuttaHitAssertion, TestCase};
    use crate::identifiers::SuttacentralUrl;

    fn test_case() -> TestCase {
        let assertions = Assertions {
            sutta_hits: SuttaHitAssertion {
                top: SuttacentralUrl::from("/mn1/en/bodhi"),
            },
        };

        TestCase {
            query: String::from("mn1"),
            description: String::from("Find with uid mn1"),
            limit: 1,
            site_language: String::from("en"),
            restrict: String::from("all"),
            selected_languages: vec![String::from("en")],
            match_partial: false,
            assertions: Some(assertions),
        }
    }

    fn search_response() -> SearchResponse {
        let json = r#"
        {
            "total": 1,
            "hits" : [
                {
                    "uid": "mn1",
                    "lang": "en",
                    "author_uid": "bodhi",
                    "url": "/mn1/en/bodhi"
                }
            ],
            "suttaplex" : [],
            "fuzzy_dictionary": []
        }
        "#;

        SearchResponse::from_json(json).unwrap()
    }

    #[test]
    fn can_assert_top_sutta_hit() {
        let test_case = test_case();
        let search_response = search_response();
        let test_case_url = test_case.assertions.unwrap().sutta_hits.top;
        assert_eq!(test_case_url, search_response.text_hits()[0]);
    }
}
