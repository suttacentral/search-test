#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct TopLevel {
        numbers: Numbers,
    }

    #[derive(Deserialize, Debug)]
    struct Numbers {
        list: Vec<i32>,
    }

    #[test]
    fn parse_all_numbers() {
        let json = r#"
        {
            "numbers": {
                "list": [1, 2, 3, 4, 5]
            }
        }
        "#;

        let top_level = serde_json::from_str::<TopLevel>(json).unwrap();
        assert_eq!(top_level.numbers.list, vec![1, 2, 3, 4, 5])
    }
}
