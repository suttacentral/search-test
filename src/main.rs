use reqwest::Error;
use reqwest::blocking::{Client, Request};

struct TestCase {
    url: String,
    query: String,
    limit: u16,
    uri_language: String,
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
            uri_language: "en".to_string(),
            restrict: "all".to_string(),
            match_partial: "false".to_string(),
            selected_languages: vec!["en".to_string()],
        }
    }
}

fn build_search_request(test_case: TestCase) -> Result<Request, Error> {
    let params = vec![
        ("limit", test_case.limit.to_string()),
        ("query", test_case.query),
        ("language", test_case.uri_language),
        ("restrict", test_case.restrict),
        ("matchpartial", test_case.match_partial),
    ];

    Client::new()
        .post(test_case.url.as_str())
        .query(&params)
        .json(&test_case.selected_languages)
        .build()
}

fn main() {
    let test_case = TestCase::default();
    let request = build_search_request(test_case).unwrap();
    let client = Client::new();
    let response = client.execute(request).unwrap();
    println!("{}", response.text().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_request_has_correct_url() {
        let test_case = TestCase {
            query: String::from("adze"),
            ..Default::default()
        };

        let request = build_search_request(test_case).unwrap();

        assert_eq!(
            request.url().as_str(),
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
    }

    #[test]
    fn search_request_has_correct_body() {
        let test_case = TestCase {
            query: String::from("adze"),
            selected_languages: vec!["en".to_string(), "pli".to_string()],
            ..Default::default()
        };

        let request = build_search_request(test_case).unwrap();
        let body = request.body().unwrap().as_bytes().unwrap();
        let body_contents = str::from_utf8(body).unwrap().to_string();

        assert_eq!(body_contents, "[\"en\",\"pli\"]");
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    struct Gadget {
        name: String,
    }

    #[test]
    fn can_use_serde() {
        let gadget_json = "{ \"name\": \"extendable legs\" }";
        let gadget: Gadget = serde_json::from_str(gadget_json).unwrap();
        assert_eq!(gadget.name, "extendable legs");
    }
}
