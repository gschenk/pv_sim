use serde::Deserialize;
use std::error::Error;
use std::fs;
use toml;

const DEFAULT_FILE: &str = "default.toml";

#[derive(Debug, Clone)]
pub struct Config {
    pub rabbit: Rabbit,
    pub time: Time,
    pub flags: Flags,
    pub random: Random,
}

#[derive(Deserialize, Debug)]
pub struct TomlConfig {
    pub rabbit: Rabbit,
    pub time: Time,
    random: Random,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rabbit {
    pub user: String,
    pub address: String,
    pub port: usize,
    pub queue: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Time {
    pub stepsize: u64,
    pub start: u64,
    pub end: u64,
    pub day: u64,
    pub year: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Random {
    pub max_power: f64,
    pub min_power: f64,
    pub sigma: f64,
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub quiet: bool,
}

// get config from command line arguments
// looking for filename only
impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let flags = flags_from_args(&args);
        let filename = file_from_args(&args);

        // read contents from config file
        let contents = readfile(&filename)
            .map_err(|e| format!("Cannot read configuration file {}. {}", filename, e))?;

        let TomlConfig {
            rabbit,
            time,
            random,
        } = detoml(&contents)
            .map_err(|e| format!("Cannot parse configuration file {}. {}", filename, e))?;

        Ok(Config {
            rabbit,
            time,
            flags,
            random,
        })
    }
}

// Processing CLI Arguments

// constructs default Flags struct and has a method for
// each field that can be toggled
impl Flags {
    // new Flags struct with all fields set to default
    fn new() -> Flags {
        return Flags { quiet: false };
    }
    fn quiet(&mut self) {
        self.quiet = true;
    }
}

// check if a string starts with a char that symbols a comand line flag
fn is_flag(s: &str) -> bool {
    return match s.chars().next() {
        Some('-') => true,
        _ => false,
    };
}

// returns first argument that is not a cli flag
// or default file if empty
fn file_from_args(args: &[String]) -> &str {
    let foo = args.iter().skip(1).find(|s| !is_flag(s));

    return match foo {
        Some(s) => s,
        _ => DEFAULT_FILE,
    };
}

// returns Flags struct with flags set for CLI flags it finds in its
// argument list
fn flags_from_args(args: &[String]) -> Flags {
    let mut flags = Flags::new();
    let _ = args.iter().skip(1).filter(|s| is_flag(s)).for_each(|s| {
        match &**s {
            "-q" => flags.quiet(),
            "--quiet" => flags.quiet(),
            _ => {}
        };
    });
    return flags;
}

// Reading and Deserializing

// read file with input data
fn readfile(file: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}

// deserialize raw input data
fn detoml(rawinput: &str) -> Result<TomlConfig, Box<dyn Error>> {
    let parsed: TomlConfig = toml::from_str(&rawinput)?;
    Ok(parsed)
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

                [time]
                stepsize = 1
                start = 0
                end = 1
                day = 120
                year = 2020

                [random]
                max_power = 9.0
                min_power = 0.5
                sigma = 5e-3
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
