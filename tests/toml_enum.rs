use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Table {
    key: String,
}

#[derive(Deserialize, Serialize)]
struct Document {
    table: Table,
}

#[test]
fn a_only() {
    let toml = r#"
     [table]
     key = "Super"
     "#;

    let table = toml::from_str::<Document>(toml).unwrap();
}
