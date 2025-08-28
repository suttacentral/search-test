pub mod results;
pub mod suite;

use crate::results::SearchResults;
use crate::suite::TestSuite;
use reqwest::Error;
use reqwest::blocking::{Client, RequestBuilder};

struct TestCase {
    url: String,
    query: String,
    limit: u16,
    site_language: String,
    restrict: String,
    match_partial: String,
    selected_languages: Vec<String>,
}

impl Default for TestCase {
    fn default() -> Self {
        TestCase {
            query: String::new(),
            url: "http://localhost/api/search/instant".to_string(),
            limit: 1,
            site_language: "en".to_string(),
            restrict: "all".to_string(),
            match_partial: "false".to_string(),
            selected_languages: vec!["en".to_string()],
        }
    }
}

impl From<TestCase> for RequestBuilder {
    fn from(value: TestCase) -> RequestBuilder {
        let params = vec![
            ("limit", value.limit.to_string()),
            ("query", value.query),
            ("language", value.site_language),
            ("restrict", value.restrict),
            ("matchpartial", value.match_partial),
        ];

        Client::new()
            .post(value.url.as_str())
            .query(&params)
            .json(&value.selected_languages)
    }
}

fn with_fuzzy_dictionary_result() -> TestCase {
    let selected_languages = vec![
        "lzh".to_string(),
        "en".to_string(),
        "pgd".to_string(),
        "kho".to_string(),
        "pli".to_string(),
        "pra".to_string(),
        "san".to_string(),
        "xct".to_string(),
        "xto".to_string(),
        "uig".to_string(),
    ];

    TestCase {
        query: "pacch".to_string(),
        selected_languages,
        match_partial: "true".to_string(),
        limit: 10,
        ..Default::default()
    }
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
    let test_case = with_fuzzy_dictionary_result();
    let request = RequestBuilder::from(test_case);
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
    use crate::suite::TestSuite;

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
