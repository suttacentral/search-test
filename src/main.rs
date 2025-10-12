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
    let test_suite = load_suite();

    match test_suite {
        Ok(test_suite) => {
            let search_service = LiveSearchService::new(test_suite.endpoint().clone());
            let runner = Runner::new(&test_suite, search_service).unwrap();

            println!("{}", test_suite.headline());

            for result in runner.run() {
                print!("{result}");
                sleep(Duration::from_millis(test_suite.delay()));
            }
        }
        Err(error) => println!("{error}"),
    }
}
