use crate::identifiers::SuttaplexUid;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SuttaplexHit {
    uid: SuttaplexUid,
}

#[derive(Deserialize, Debug)]
struct SuttaplexHits {
    suttaplex: Vec<SuttaplexHit>,
}

pub fn suttaplex_results(json: &str) -> Result<Vec<SuttaplexUid>> {
    let hits: SuttaplexHits = serde_json::from_str(json)?;
    let uids = hits.suttaplex.iter().map(|hit| hit.uid.clone()).collect();
    Ok(uids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_results() {
        let json = r#"
        {
            "suttaplex": []
        }
        "#;

        assert_eq!(suttaplex_results(json).unwrap(), Vec::new())
    }

    #[test]
    fn single_result() {
        let json = r#"
        {
            "suttaplex": [
                {
                    "uid": "mn1"
                }
            ]
        }
        "#;

        assert_eq!(
            suttaplex_results(json).unwrap(),
            vec![SuttaplexUid::from("mn1")]
        )
    }

    #[test]
    fn two_results() {
        let json = r#"
        {
            "suttaplex": [
                {
                    "uid": "mn1"
                },
                {
                    "uid": "mn2"
                }
            ]
        }
        "#;

        assert_eq!(
            suttaplex_results(json).unwrap(),
            vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")]
        )
    }
}
