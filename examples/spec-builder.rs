use search_test::spec::SettingsBuilder;

fn main() {
    let settings: String = SettingsBuilder::new()
        .endpoint(String::from("http://localhost/api/search/instant"))
        .limit(50)
        .build();
    println!("{settings}");
}
