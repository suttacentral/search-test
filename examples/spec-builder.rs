use search_test::spec::SettingsBuilder;

fn main() {
    let settings: String = SettingsBuilder::new()
        .endpoint("http://localhost/api/search/instant")
        .limit(50)
        .site_language("en")
        .restrict("all")
        .selected_languages(vec![
            "lzh", "en", "pgd", "kho", "pli", "pra", "san", "xct", "xto", "uig",
        ])
        .build();
    println!("{settings}");
}
