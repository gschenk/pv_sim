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
    pub rabbit: Rabbit,
    pub time: Time,
}

#[derive(Deserialize, Debug)]
pub struct Rabbit {
    pub user: String,
    pub address: String,
    pub port: usize,
    pub queue: String,
}

#[derive(Deserialize, Debug)]
pub struct Time {
    pub stepsize: u64,
    pub start: u64,
    pub end: u64,
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
                [rabbit]
                user = "foo"
                address = '0.0.0.0'
                port = 5672
                queue = "default"
            "#;
        let expected = detoml(&a).unwrap();
        println!("{:?}", expected);
        assert_eq!(expected.rabbit.address, "0.0.0.0".to_string());
        assert_eq!(expected.rabbit.port, 5672);
    }

    #[test]
    fn default_config_consistent() {
        let config = Config::new(&["bin".to_string()]);
        println!("{:?}", config);
        assert!(config.is_ok());
    }
}
