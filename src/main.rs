struct SearchTestCase {}

fn main() {
    println!("Run system tests on the SuttaCentral search engine.");
    let _case = SearchTestCase {};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_a_search_test_case() {
        let _case = SearchTestCase {};
    }
}
