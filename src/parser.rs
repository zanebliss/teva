use serde::Deserialize;
use std::error::Error;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub test: Test,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub pattern: String,
    pub setup: Option<Setup>,
    pub run: Option<Run>,
}

#[derive(Debug, Deserialize)]
pub struct Setup {
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub struct Run {
    pub command: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Step {
    pub name: String,
    pub command: String,
    pub args: Option<Vec<String>>,
}

pub fn parse_config_file(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    Ok(toml::from_str(contents.as_str()).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_config_file {
        use super::parse_config_file;
        use std::error::Error;
        use std::fs::File;
        use std::io::Write;

        #[test]
        fn with_config_file_is_ok() -> Result<(), Box<dyn Error>> {
            let tmp_dir = tempfile::tempdir()?;
            let file_path = tmp_dir.path().join(".teva.toml");
            let mut tmp_file = File::create(&file_path)?;
            let toml = r#"
                [test]
                pattern = "_spec.rb"

                [test.setup]
                steps = [
                    { name = "yarn", command = "yarn"},
                    { name = "db:migrate", command = "rails", args = ["RAILS_ENV=test", "db:migrate"]}
                ]

                [test.run]
                name = "rspec"
                command = "bundle"
                args = ["exec", "rspec"]
                "#;
            writeln!(tmp_file, "{}", toml)?;

            let config = parse_config_file(file_path);

            assert!(config.is_ok());

            let result = config?;
            let test = result.test.setup.unwrap().steps;
            let run = result.test.run.unwrap();

            assert_eq!("_spec.rb", result.test.pattern);
            assert_eq!(2, test.len());
            assert_eq!("_spec.rb", result.test.pattern);
            assert_eq!("yarn", test.first().unwrap().command);
            assert_eq!("rails", test.get(1).unwrap().command);
            assert_eq!(
                ["RAILS_ENV=test".to_string(), "db:migrate".to_string()].to_vec(),
                test.get(1).unwrap().args.clone().unwrap()
            );

            assert_eq!("bundle", run.command);
            assert_eq!(
                ["exec".to_string(), "rspec".to_string()].to_vec(),
                run.args.clone().unwrap()
            );

            Ok(())
        }

        #[test]
        #[should_panic]
        fn without_config_file_panics() {
            let tmp_dir = tempfile::tempdir().unwrap();
            let file_path = tmp_dir.path().join("");

            parse_config_file(file_path).unwrap();
        }
    }
}
