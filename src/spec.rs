#[derive(Default)]
pub struct SettingsBuilder {
    endpoint: Option<String>,
}

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder { endpoint: None }
    }

    pub fn endpoint(mut self, endpoint: String) -> SettingsBuilder {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn build(self) -> String {
        let mut output = String::from("settings:\n");
        output.push_str("    endpoint: \"http://localhost/api/search/instant\"");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ENDPOINT: &str = "http://localhost/api/search/instant";

    #[test]
    fn create_builder_with_endpoint() {
        let settings: String = SettingsBuilder::new()
            .endpoint(String::from(ENDPOINT))
            .build();

        let mut expected = String::new();
        expected.push_str("settings:\n");
        expected.push_str("    endpoint: \"http://localhost/api/search/instant\"");
        assert_eq!(settings, expected);
    }
}
