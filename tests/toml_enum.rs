use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum SubKey {
    A { x: String },
}

#[derive(Deserialize, Serialize)]
struct Table {
    key: SubKey,
}

#[derive(Deserialize, Serialize)]
struct Document {
    table: Table,
}

#[test]
fn a_only() {
    let toml = r#"
     [table]
     key.a.x = "Super"
     "#;

    let table = toml::from_str::<Document>(toml).unwrap();
}
