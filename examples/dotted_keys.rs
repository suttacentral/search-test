use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Dotted {
    key_a: Option<String>,
    key_b: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Table {
    dotted: Dotted,
}

#[derive(Debug, Deserialize)]
struct TomlDocument {
    table: Table,
}

fn main() {
    let dotted: Result<TomlDocument, toml::de::Error> = toml::from_str(
        r#"
            [table]
            dotted.key_a = "formaldahyde"
            dotted.key_b = "ostrich"
        "#,
    );

    println!("{dotted:#?}");
}
