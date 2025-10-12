mod category_search;
mod defaults;
mod expected;
mod identifiers;
mod report;
mod response;
mod run;
mod search_service;
mod test_case;
mod test_result;
mod test_suite;

use crate::run::Runner;
use crate::test_suite::TestSuite;
use anyhow::{Result, anyhow};
use search_service::LiveSearchService;
use std::thread::sleep;
use std::time::Duration;

fn get_file_name(args: Vec<String>) -> Result<String> {
    match args.len() {
        1 => Err(anyhow!("No test suite file name provided")),
        2 => Ok(args[1].clone()),
        _ => Err(anyhow!(
            "Too many arguments. Only one required: the suite file name"
        )),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = get_file_name(args);
    match file_name {
        Ok(file_name) => println!("{file_name}"),
        Err(error) => println!("{}", error),
    }
    let toml = std::fs::read_to_string("test-cases/examples.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();
    let search_service = LiveSearchService::new(suite.endpoint().clone());
    let runner = Runner::new(&suite, search_service).unwrap();

    println!("{}", suite.headline());

    for result in runner.run() {
        print!("{result}");
        sleep(Duration::from_millis(suite.delay()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arguments_provided() {
        let args = vec![String::from("search-test")];
        let error = get_file_name(args).unwrap_err();
        assert_eq!(error.to_string(), "No test suite file name provided");
    }

    #[test]
    fn file_name_provided() {
        let args = vec![
            String::from("search-test"),
            String::from("test_cases/example.toml"),
        ];
        let file_name = get_file_name(args).unwrap();
        assert_eq!(
            file_name.to_string(),
            String::from("test_cases/example.toml")
        )
    }

    #[test]
    fn too_many_arguments() {
        let args = vec![
            String::from("search-test"),
            String::from("test_cases/example.toml"),
            String::from("another-argument"),
        ];
        let error = get_file_name(args).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Too many arguments. Only one required: the suite file name"
        );
    }
}
