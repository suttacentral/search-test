use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Sufficient { minimum: usize, actual: usize },
    TooLow { minimum: usize, actual: usize },
    NotFound { minimum: usize },
}

impl Rank {
    pub fn new(minimum: usize, actual: Option<usize>) -> Self {
        match actual {
            None => Rank::NotFound { minimum },
            Some(actual) => match minimum.cmp(&actual) {
                Ordering::Greater | Ordering::Equal => Self::Sufficient { minimum, actual },
                Ordering::Less => Self::TooLow { minimum, actual },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_not_found() {
        assert_eq!(Rank::new(3, None), Rank::NotFound { minimum: 3 });
    }

    #[test]
    fn rank_sufficient() {
        assert_eq!(
            Rank::new(3, Some(3)),
            Rank::Sufficient {
                minimum: 3,
                actual: 3
            }
        );
    }

    #[test]
    fn rank_too_low() {
        assert_eq!(
            Rank::new(3, Some(4)),
            Rank::TooLow {
                minimum: 3,
                actual: 4
            }
        );
    }
}
