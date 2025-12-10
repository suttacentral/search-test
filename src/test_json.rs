pub const NO_RESULTS_JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : []
    }
    "#;

pub const SUTTAPLEX_MN1_JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : [ { "uid": "mn1" } ]
    }
    "#;

pub const SUTTAPLEX_MN_FIRST_THREE_JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : [
            { "uid": "mn1" },
            { "uid": "mn2" },
            { "uid": "mn3" }
        ]
    }
    "#;
