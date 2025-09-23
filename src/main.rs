mod defaults;
mod expected;
mod identifiers;
mod request;
mod response;
mod run;
mod search_service;
mod test_case;
mod test_result;
mod test_suite;

use crate::run::Runner;
use crate::test_suite::TestSuite;
use search_service::LiveSearchEngine;

fn main() {
    let toml = std::fs::read_to_string("test-cases/play.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();
    let search_engine = LiveSearchEngine::new(suite.endpoint().clone());
    let runner = Runner::new(suite, search_engine).unwrap();

    for result in runner.run() {
        println!("{result:?}");
    }
}
