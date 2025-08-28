pub mod results;
pub mod suite;

use crate::results::SearchResults;
use crate::suite::TestSuite;
use reqwest::Error;
use reqwest::blocking::{Client, RequestBuilder};

fn test_suite() -> TestSuite {
    TestSuite::load_from_string(
        r#"
            [settings]
            endpoint = "http://localhost/api/search/instant"

            [[test-case]]
            description = "Search for the metta sutta in English and Pali"
            query = "adze"
            limit = 1
            site-language = "en"
            restrict = "all"
            match-partial=false
            selected-languages = ["en", "pli"]
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

fn main() {
    let suite = test_suite();
    let test_case = suite.test_cases().unwrap().first().unwrap().clone();
    let request = build_request(suite.endpoint(), test_case);
    let response = request.send().unwrap();
    let results: Result<SearchResults, Error> = response.json();

    match results {
        Ok(parsed_results) => {
            println!("{} results", parsed_results.total);
            println!("{} hits", parsed_results.hits.len());
            for hit in parsed_results.hits {
                println!("{hit}");
            }
            for suttaplex in parsed_results.suttaplex {
                println!("Suttaplex result: {}", suttaplex.uid)
            }
            for fuzzy in parsed_results.fuzzy_dictionary {
                println!("Fuzzy dictionary result: {}", fuzzy.url)
            }
        }
        Err(error) => {
            println!("An error occurred parsing response.");
            println!("{error:?}");
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
}
