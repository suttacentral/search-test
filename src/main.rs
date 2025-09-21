mod defaults;
mod expected;
mod identifiers;
mod request;
mod response;
mod run;
mod test_case;
mod test_suite;

use crate::request::build;
use crate::response::SearchResponse;
use crate::test_suite::TestSuite;
use anyhow::Result;
use reqwest::Error;

fn main() {
    let toml = std::fs::read_to_string("test-cases/play.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();

    for test_case in suite.test_cases() {
        let request = build(suite.endpoint(), test_case.unwrap());
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
