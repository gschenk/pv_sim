use serde::Deserialize;
use std::error::Error;
use std::fs;
use toml;

const DEFAULT_FILE: &str = "default.toml";

// read file with input data
fn readfile(file: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}

// deserialize raw input data
fn detoml(rawinput: &str) -> Result<Config, Box<dyn Error>> {
    let parsed: Config = toml::from_str(&rawinput)?;
    Ok(parsed)
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub address: String,
}

// get config from command line arguments
// looking for filename only
impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let filename = if args.len() > 1 {
            &args[1]
        } else {
            DEFAULT_FILE
        };

        // read contents from config file
        let contents = readfile(&filename)
            .map_err(|e| format!("Cannot read configuration file {}. {}", filename, e))?;

        let config = detoml(&contents)
            .map_err(|e| format!("Cannot parse configuration file {}. {}", filename, e))?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detoml() {
        let a: &str = r#"
                address = '0.0.0.0'
            "#;
        let expected = detoml(&a).unwrap();
        println!("{:?}", expected);
        assert_eq!(expected.address, "0.0.0.0".to_string());
    }

    #[test]
    fn default_config_consistent() {
        let config = Config::new(&["bin".to_string()]);
        println!("{:?}", config);
        assert!(config.is_ok());
    }
}
