use std::fmt::Write as _;

#[derive(Default)]
pub struct SettingsBuilder {
    endpoint: Option<String>,
    limit: Option<usize>,
    site_language: Option<String>,
}

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder {
            endpoint: None,
            limit: None,
            site_language: None,
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

    pub fn site_language(mut self, site_language: String) -> SettingsBuilder {
        self.site_language = Some(site_language);
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

        if let Some(site_language) = self.site_language {
            writeln!(&mut output, "    site-language: \"{site_language}\"")
                .expect("Building failed.")
        }
        output
    }
}
