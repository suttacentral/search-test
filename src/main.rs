struct SearchTestCase {
    query: String,
    limit: u16,
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
    let uri_language = "en";
    let restrict = "all";
    let match_partial = "false";

    let uri = format!(
        "http://{}{}?limit={}&query={}&language={}&restrict={}&matchpartial={}",
        host, path, test_case.limit, test_case.query, uri_language, restrict, match_partial,
    );
    SearchRequest { uri }
}

fn main() {
    println!("Run system tests on the SuttaCentral search engine.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_assign_a_query_to_test_case() {
        let case = SearchTestCase {
            query: String::from("adze"),
            limit: 1,
        };

        assert_eq!(case.query, "adze");
    }

    #[test]
    fn can_add_query_to_request() {
        let case = SearchTestCase {
            query: String::from("adze"),
            limit: 1,
        };

        let request = create_request(case);
        assert_eq!(
            request.uri,
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
    }

    struct SearchClientStub {}

    impl SearchClientStub {
        fn send(self, request: SearchRequest) -> SearchResponse {
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
