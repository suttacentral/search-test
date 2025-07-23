struct SearchTestCase {
    query: String,
    limit: u16,
    uri_language: String,
    restrict: String,
    match_partial: String,
}

impl Default for SearchTestCase {
    fn default() -> Self {
        SearchTestCase {
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

struct SearchResponse {
    top_result: String,
}

fn create_request(test_case: SearchTestCase) -> SearchRequest {
    let host = "localhost";
    let path = "/api/search/instant";

    let uri = format!(
        "http://{}{}?limit={}&query={}&language={}&restrict={}&matchpartial={}",
        host,
        path,
        test_case.limit,
        test_case.query,
        test_case.uri_language,
        test_case.restrict,
        test_case.match_partial,
    );

    SearchRequest { uri }
}

fn main() {
    let test_case = SearchTestCase::default();
    let request = create_request(test_case);
    let response = SearchResponse {
        top_result: "mn1".to_string(),
    };
    println!("Request URI: {}", request.uri);
    println!("Top result was {}", response.top_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_test_case_limit_defaults_to_one() {
        let case = SearchTestCase::default();
        assert_eq!(case.limit, 1);
    }

    #[test]
    fn can_add_query_to_request() {
        let case = SearchTestCase {
            query: String::from("adze"),
            ..Default::default()
        };

        let request = create_request(case);
        assert_eq!(
            request.uri,
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
    }

    struct SearchClientStub {}

    impl SearchClientStub {
        fn send(self, _request: SearchRequest) -> SearchResponse {
            SearchResponse {
                top_result: String::from("example"),
            }
        }
    }

    #[test]
    fn can_send_a_request() {
        let request = SearchRequest {
            uri: String::from("http://example.com"),
        };
        let client = SearchClientStub {};
        let response = client.send(request);
        assert_eq!(response.top_result, "example");
    }
}
