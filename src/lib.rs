use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must enter the query and file");
        }
        let query = args[1].clone();
        let file = args[2].clone();

        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            file: file,
            case_sensitive: is_case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in result {
        println!("With text:\n{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
