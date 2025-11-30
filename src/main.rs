mod category_search;
mod defaults;
mod expected;
mod file_load;
mod identifiers;
mod outcome;
mod rank;
mod report;
mod request;
mod response;
mod result_count;
mod run;
mod search_service;
mod summary;
mod test_case;
#[cfg(test)]
mod test_json;
mod test_result;
mod test_suite;
mod timed_response;
mod timed_search_results;

use crate::file_load::load_suite;
use crate::result_count::ResultCount;
use crate::run::Runner;
use crate::summary::Summary;
use anyhow::Result;
use search_service::LiveSearchService;
use std::thread::sleep;
use std::time::Duration;

fn run_application() -> Result<ResultCount> {
    let test_suite = load_suite()?;
    let search_service = LiveSearchService::new(test_suite.endpoint().clone());
    let runner = Runner::new(&test_suite, search_service)?;

    println!("{}", test_suite.headline());
    println!();

    let mut result_count = ResultCount::new();

    for result in runner.run() {
        print!("{result}");
        sleep(Duration::from_millis(test_suite.delay()));
        result_count.add(&Summary::from(&result.outcome));
    }

    Ok(result_count)
}

fn main() {
    match run_application() {
        Ok(count) => println!("{count}"),
        Err(error) => println!("{error:#}"),
    }
}
