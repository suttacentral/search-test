#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Unsorted {
        numbers: Vec<u32>,
    }

    #[test]
    fn parse_all_numbers() {
        let json = r#"
        {
            "numbers": [1, 2, 3, 4, 5]
        }
        "#;

        let top_level = serde_json::from_str::<Unsorted>(json).unwrap();
        assert_eq!(top_level.numbers, vec![1, 2, 3, 4, 5])
    }
}
