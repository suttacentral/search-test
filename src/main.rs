mod defaults;
mod expected;
mod identifiers;
mod request;
mod response;
mod run;
mod test_case;
mod test_suite;

use crate::run::Runner;
use crate::test_suite::TestSuite;

fn main() {
    let toml = std::fs::read_to_string("test-cases/play.toml").unwrap();
    let suite = TestSuite::load_from_string(toml.as_str()).unwrap();
    let runner = Runner::new(suite);

    for result in runner.run() {
        println!("{result:?}");
    }
}
