use search_test::spec::SettingsBuilder;

fn main() {
    let settings: String = SettingsBuilder::new()
        .endpoint("http://localhost/api/search/instant")
        .limit(50)
        .site_language("en")
        .restrict("all")
        .build();
    println!("{settings}");
}
