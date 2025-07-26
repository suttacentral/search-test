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

struct SearchRequest {
    uri: String,
}

fn create_request(target: Target, test_case: TestCase) -> SearchRequest {
    let uri = format!(
        "{}?limit={}&query={}&language={}&restrict={}&matchpartial={}",
        target.endpoint,
        test_case.limit,
        test_case.query,
        test_case.uri_language,
        test_case.restrict,
        test_case.match_partial,
    );

    SearchRequest { uri }
}

fn main() {
    let test_case = TestCase::default();

    let target = Target {
        name: String::from("dev"),
        endpoint: String::from("http://localhost/api/search/instant"),
    };

    println!("Creating request for {} target", target.name);

    let request = create_request(target, test_case);
    println!("Search URL is {}", request.uri);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_test_case_limit_defaults_to_one() {
        let case = TestCase::default();
        assert_eq!(case.limit, 1);
    }

    #[test]
    fn can_add_query_to_request() {
        let target = Target {
            name: String::from("dev"),
            endpoint: String::from("http://localhost/api/search/instant"),
        };

        let test_case = TestCase {
            query: String::from("adze"),
            ..Default::default()
        };

        let request = create_request(target, test_case);
        assert_eq!(
            request.uri,
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
    }

    #[test]
    fn can_use_reqwest() {
        let _result = reqwest::blocking::get("http://localhost");
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

    #[test]
    fn use_request_builder() {
        let params = vec![("venom", "deadly"), ("teeth", "pointy")];
        let client = reqwest::blocking::Client::new();
        let builder = client.post("http://reptiles.com/api/snake").query(&params);
        let request = builder.build().unwrap();
        let url = request.url();
        assert_eq!(
            url.as_str(),
            "http://reptiles.com/api/snake?venom=deadly&teeth=pointy"
        )
    }
}
