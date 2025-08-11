use std::fmt::Write as _;

#[derive(Default)]
pub struct SettingsBuilder {
    endpoint: Option<String>,
    limit: Option<usize>,
}

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder {
            endpoint: None,
            limit: None,
        }
    }

    pub fn endpoint(mut self, endpoint: String) -> SettingsBuilder {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn limit(mut self, limit: usize) -> SettingsBuilder {
        self.limit = Some(limit);
        self
    }

    pub fn build(self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "settings: ").expect("Building failed.");

        if let Some(endpoint) = self.endpoint {
            writeln!(&mut output, "    endpoint: \"{endpoint}\"").expect("Building failed.");
        }

        if let Some(limit) = self.limit {
            writeln!(&mut output, "    limit: {limit}").expect("Building failed.");
        }
        output
    }
}
