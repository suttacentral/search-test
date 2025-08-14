use saphyr::Yaml::Value;
use saphyr::{LoadableYamlNode, Scalar, Yaml};
use search_test::builders::SettingsBuilder;
use std::borrow::Cow;

fn main() {
    let settings: String = SettingsBuilder::new()
        .endpoint("http://localhost/api/search/instant")
        .limit(50)
        .site_language("en")
        .restrict("all")
        .selected_languages(vec![
            "lzh", "en", "pgd", "kho", "pli", "pra", "san", "xct", "xto", "uig",
        ])
        .match_partial(true)
        .build();

    let docs = Yaml::load_from_str(settings.as_str()).unwrap();

    let settings_map = &docs[0]["settings"];
    let endpoint = &settings_map["endpoint"];
    let endpoint_value = endpoint.as_str().unwrap();
    println!("Endpoint 1: {endpoint_value}");
    // dbg!(settings_map);

    match settings_map {
        Yaml::Mapping(mapping) => {
            let key = Yaml::Value(Scalar::String(Cow::from("endpoint")));
            let endpoint_value = &mapping.get(&key);
            let endpoint = endpoint_value.unwrap().as_str().unwrap();
            println!("Endpoint: 2: {endpoint}")
        }
        _ => {
            println!("Not a mapping");
        }
    }
}
