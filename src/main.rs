struct SearchTestCase {
    query: String,
}

struct SearchRequest {
    uri: String,
}

fn create_request(test_case: SearchTestCase) -> SearchRequest {
    let uri = format!(
        "http://localhost/api/search/instant?limit=50&query={}&language=en&restrict=all&matchpartial=false",
        test_case.query
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
        };

        assert_eq!(case.query, "adze")
    }

    #[test]
    fn can_add_query_to_request() {
        let case = SearchTestCase {
            query: String::from("adze"),
        };

        let request = create_request(case);
        assert_eq!(
            request.uri,
            "http://localhost/api/search/instant?limit=50&query=adze&language=en&restrict=all&matchpartial=false"
        )
    }
}
