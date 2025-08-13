use search_test::spec::SettingsBuilder;

fn main() {
    let settings: String = SettingsBuilder::new()
        .endpoint(String::from("http://localhost/api/search/instant"))
        .limit(50)
        .site_language(String::from("en"))
        .restrict(String::from("all"))
        .build();
    println!("{settings}");
}
