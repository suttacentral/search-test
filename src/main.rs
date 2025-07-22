struct SearchTestCase {
    query: String,
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

        assert_eq!(case.query, "adzexxx")
    }
}
