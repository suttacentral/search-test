mod category_search;
mod defaults;
mod expected;
mod identifiers;
mod response;
mod run;
mod search_service;
mod test_case;
mod test_result;
mod test_suite;

use crate::run::Runner;
use crate::test_suite::TestSuite;
use search_service::LiveSearchService;

fn main() {
    let toml = std::fs::read_to_string("test-cases/play.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();
    let search_service = LiveSearchService::new(suite.endpoint().clone());
    let runner = Runner::new(suite, search_service).unwrap();

    for result in runner.run() {
        //println!("{result:?}");
    }
}
