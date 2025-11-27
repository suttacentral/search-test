use crate::test_case::TestCase;
use anyhow::{Context, Result};
use reqwest::blocking::{Client, RequestBuilder, Response};

pub struct Request {
    endpoint: String,
    test_case: TestCase,
}

impl Request {
    pub fn new(endpoint: String, test_case: &TestCase) -> Request {
        Request {
            endpoint: endpoint,
            test_case: test_case.clone(),
        }
    }

    pub fn send(&self) -> Result<Response> {
        self.build_request()
            .send()
            .context("Error sending HTTP request")
    }

    fn build_request(&self) -> RequestBuilder {
        Client::new()
            .post(self.endpoint.as_str())
            .query(&self.parameters())
            .json(&self.test_case.selected_languages)
    }

    fn parameters(&self) -> Vec<(String, String)> {
        vec![
            ("limit".to_string(), self.test_case.limit.to_string()),
            ("query".to_string(), self.test_case.query.to_string()),
            (
                "language".to_string(),
                self.test_case.site_language.to_string(),
            ),
            ("restrict".to_string(), self.test_case.restrict.to_string()),
            (
                "matchpartial".to_string(),
                self.test_case.match_partial.to_string(),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case() -> TestCase {
        TestCase {
            description: String::from("The Simile of the Adze"),
            query: String::from("adze"),
            limit: 1,
            site_language: String::from("en"),
            restrict: String::from("all"),
            match_partial: false,
            selected_languages: vec![String::from("en"), String::from("pli")],
            expected: None,
        }
    }

    #[test]
    fn builds_url() {
        let url = Request::new(
            String::from("http://localhost/api/search/instant"),
            &test_case(),
        )
        .build_request()
        .build()
        .unwrap()
        .url()
        .to_string();

        assert_eq!(
            url,
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
    }

    #[test]
    fn has_correct_body() {
        let body_string = str::from_utf8(
            Request::new(
                String::from("http://localhost/api/search/instant"),
                &test_case(),
            )
            .build_request()
            .build()
            .unwrap()
            .body()
            .unwrap()
            .as_bytes()
            .unwrap(),
        )
        .unwrap()
        .to_string();

        assert_eq!(body_string, "[\"en\",\"pli\"]");
    }
}
