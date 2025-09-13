#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Unsorted {
        numbers: Vec<u32>,
    }

    fn json() -> String {
        String::from(r#" { "numbers": [1, 2, 3, 4, 5] } "#)
    }

    #[test]
    fn parse_unsorted() {
        let unsorted = serde_json::from_str::<Unsorted>(json().as_str()).unwrap();
        assert_eq!(unsorted.numbers, vec![1, 2, 3, 4, 5])
    }
}
