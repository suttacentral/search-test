use search_test::spec::SettingsBuilder;

fn main() {
    let settings: String = SettingsBuilder::new()
        .endpoint(String::from("http://localhost/api/search/instant"))
        .build();
    println!("{settings}");
}
