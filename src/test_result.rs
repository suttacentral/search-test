use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use std::time::Duration;

#[derive(Debug)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
}

impl TestResult {
    pub fn new(test_case: &TestCase, timed: &TimedSearchResults) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed: timed.elapsed,
        }
    }
}

struct CategorySearch<C>
where
    C: PartialEq,
{
    id: C,
    sequence: Vec<C>,
}

impl<C: PartialEq> CategorySearch<C> {
    fn found(&self) -> bool {
        self.sequence.contains(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::identifiers::SuttaplexUid;
    use crate::test_result::CategorySearch;

    #[test]
    fn suttaplex_found() {
        let search = CategorySearch {
            id: SuttaplexUid::from("mn1"),
            sequence: vec![SuttaplexUid::from("mn1")],
        };

        assert!(search.found())
    }
}
