use serde::Deserialize;

#[derive(Deserialize)]
struct Nested {
    a: String,
    b: String,
}

#[derive(Deserialize)]
struct Table {
    nested: Nested,
}

#[derive(Deserialize)]
struct Document {
    table: Table,
}

#[test]
fn nested_keys() {
    let toml = r#"
    [table]
    nested.a = "Super"
    nested.b = "Glue"
    "#;

    let table = toml::from_str::<Document>(toml).unwrap();
}
