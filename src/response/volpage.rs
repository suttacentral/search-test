use crate::identifiers::VolpageReference;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VolpageHit {
    volpage: VolpageReference,
}

#[derive(Deserialize, Debug)]
struct VolpageHits {
    hits: Vec<VolpageHit>,
}

pub fn volpage_results(json: &str) -> Result<Vec<VolpageReference>> {
    let hits: VolpageHits = serde_json::from_str(json)?;
    let references = hits.hits.iter().map(|hit| hit.volpage.clone()).collect();
    Ok(references)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_error_when_json_is_bad() {
        let error = volpage_results("Not json").unwrap_err();
        assert_eq!(error.to_string(), "expected value at line 1 column 1")
    }

    #[test]
    fn no_results() {
        let json = r#"
        {
            "hits": []
        }
        "#;

        assert_eq!(volpage_results(json).unwrap(), Vec::new())
    }

    #[test]
    fn single_result() {
        let json = r#"
        {
            "hits": [
                {
                    "volpage": "PTS SN ii 1"
                }
            ]
        }
        "#;

        assert_eq!(
            volpage_results(json).unwrap(),
            vec![VolpageReference::from("PTS SN ii 1")]
        )
    }

    #[test]
    fn two_results() {
        let json = r#"
        {
            "hits": [
                {
                    "volpage": "PTS SN ii 1"
                },
                {
                    "volpage": "PTS SN ii 2"
                }
            ]
        }
        "#;

        assert_eq!(
            volpage_results(json).unwrap(),
            vec![
                VolpageReference::from("PTS SN ii 1"),
                VolpageReference::from("PTS SN ii 2")
            ]
        )
    }
}
