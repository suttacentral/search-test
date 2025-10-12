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
use anyhow::Result;
use search_service::LiveSearchService;
use std::thread::sleep;
use std::time::Duration;

fn run_application() -> Result<()> {
    let test_suite = load_suite()?;
    let search_service = LiveSearchService::new(test_suite.endpoint().clone());
    let runner = Runner::new(&test_suite, search_service)?;

    println!("{}", test_suite.headline());

    for result in runner.run() {
        print!("{result}");
        sleep(Duration::from_millis(test_suite.delay()));
    }

    Ok(())
}

fn main() {
    if let Err(error) = run_application() {
        println!("{error}")
    }
}
