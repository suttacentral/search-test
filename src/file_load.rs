use anyhow::{Context, Result, anyhow};

fn get_file_name(args: Vec<String>) -> Result<String> {
    match args.len() {
        1 => Err(anyhow!("No test suite file name provided")),
        2 => Ok(args[1].clone()),
        _ => Err(anyhow!(
            "Too many arguments. Only one required: the suite file name"
        )),
    }
}

pub fn get_toml() -> Result<String> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = get_file_name(args)?;
    std::fs::read_to_string(file_name).context("Error reading file")
}

#[cfg(test)]
mod tests {
    use crate::file_load::get_file_name;

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
