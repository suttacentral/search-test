use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchResults {
    pub total: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn can_get_search_results_from_json_file() {
        let path = "examples/metta.json";
        let data = fs::read_to_string(path).unwrap();
        let results: SearchResults = serde_json::from_str(data.as_str()).unwrap();
        assert_eq!(results.total, 80);
    }
}
