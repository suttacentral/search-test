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

#[derive(Debug, Clone, PartialEq)]
struct ResultCount {
    passed: usize,
    failed: usize,
    error: usize,
}

impl ResultCount {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            error: 0,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn initialise_result_count() {
        let count = ResultCount::new();
        assert_eq!(
            count,
            ResultCount {
                passed: 0,
                failed: 0,
                error: 0
            }
        );
    }
}
