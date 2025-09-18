mod act;
mod arrange;
mod defaults;
mod details;
mod identifiers;
mod request;

use crate::act::SearchResponse;
use crate::arrange::TestSuite;
use crate::request::build;
use anyhow::Result;
use reqwest::Error;

fn main() {
    let toml = std::fs::read_to_string("test-cases/play.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();
    let test_cases = suite.test_cases().unwrap();

    for test_case in test_cases {
        let request = build(suite.endpoint(), test_case);
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
