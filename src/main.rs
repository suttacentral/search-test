struct Target {
    name: String,
    endpoint: String,
}

struct TestCase {
    query: String,
    limit: u16,
    uri_language: String,
    restrict: String,
    match_partial: String,
}

impl Default for TestCase {
    fn default() -> Self {
        TestCase {
            query: String::new(),
            limit: 1,
            uri_language: "en".to_string(),
            restrict: "all".to_string(),
            match_partial: "false".to_string(),
        }
    }
}

fn build_search_request(target: &Target, test_case: TestCase) -> reqwest::blocking::Request {
    let uri = format!(
        "{}?limit={}&query={}&language={}&restrict={}&matchpartial={}",
        target.endpoint,
        test_case.limit,
        test_case.query,
        test_case.uri_language,
        test_case.restrict,
        test_case.match_partial,
    );

    let params = vec![
        ("limit", "1"),
        ("query", "adze"),
        ("language", "en"),
        ("restrict", "all"),
        ("matchpartial", "false"),
    ];

    let client = reqwest::blocking::Client::new();
    let builder = client.post(target.endpoint.as_str()).query(&params);
    builder.build().unwrap()
}

fn main() {
    let test_case = TestCase::default();

    let target = Target {
        name: String::from("dev"),
        endpoint: String::from("http://localhost/api/search/instant"),
    };

    let request = build_search_request(&target, test_case);
    println!("Created request for {} target", target.name);
    println!("Search URL is {}", request.url().as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_search_request_for_dev_server() {
        let target = Target {
            name: String::from("dev"),
            endpoint: String::from("http://localhost/api/search/instant"),
        };

        let test_case = TestCase {
            query: String::from("adze"),
            ..Default::default()
        };

        let request = build_search_request(&target, test_case);

        assert_eq!(
            request.url().as_str(),
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
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
