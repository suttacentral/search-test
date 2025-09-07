use crate::identifiers::{SuttaplexUid, TextUrl};

enum Result {
    Text { url: TextUrl },
    Dictionary { url: TextUrl },
    Suttaplex { uid: SuttaplexUid },
}

enum Rank {
    Top,
    InTopFive,
    InTopTen,
    InTopN { n: u8 },
}

struct Assertion {
    expected: Result,
    rank: Rank,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::TextUrl;

    #[test]
    fn assert_text_hits_match() {
        let actual = TextUrl::from("/mn1/en/bodhi");
        let expected = TextUrl::from("/mn1/en/bodhi");
        assert_eq!(actual, expected)
    }

    #[test]
    fn create_text_assertion() {
        let assertion = Assertion {
            expected: Result::Text {
                url: TextUrl::from("/mn1/en/bodhi"),
            },
            rank: Rank::Top,
        };
    }
}
