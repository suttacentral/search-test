use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
enum SubKey {
    A { a: String },
    B { b: String },
}

#[derive(Deserialize, Serialize)]
struct Table {
    key: Option<SubKey>,
    other_key: String,
}

#[derive(Deserialize, Serialize)]
struct Document {
    table: Table,
}

#[test]
fn a_only() {
    let toml = r#"
     [table]
     key.a = "Super"
     other_key = "Glue"
     "#;

    let table = toml::from_str::<Document>(toml).unwrap();
}

#[test]
fn b_only() {
    let toml = r#"
     [table]
     key.b = "Super"
     other_key = "Glue"
     "#;

    let table = toml::from_str::<Document>(toml).unwrap();
}

#[test]
fn neither() {
    let toml = r#"
     [table]
     other_key = "Glue"
     "#;

    let table = toml::from_str::<Document>(toml).unwrap();
}
