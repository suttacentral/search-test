mod category_search;
mod defaults;
mod expected;
mod file_load;
mod identifiers;
mod report;
mod response;
mod run;
mod search_service;
mod test_case;
mod test_result;
mod test_suite;

use crate::file_load::load_suite;
use crate::run::Runner;
use search_service::LiveSearchService;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let suite = load_suite().unwrap();
    let search_service = LiveSearchService::new(suite.endpoint().clone());
    let runner = Runner::new(&suite, search_service).unwrap();

    println!("{}", suite.headline());

    for result in runner.run() {
        print!("{result}");
        sleep(Duration::from_millis(suite.delay()));
    }
}
