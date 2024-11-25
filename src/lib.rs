use std::error::Error;
use std::{fs, result};

const RED: &str = "\x1b[31m";    // Red text
const UNDERLINE: &str = "\x1b[4m"; // Underline
const RESET: &str = "\x1b[0m";    // Reset all formatting

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

fn normal_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


fn ignore_case_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }

        let mut query = String::new();
        let mut file_path = String::new();
        let mut ignore_case = false;

        for (i, arg) in args.iter().enumerate() {
            if arg == "-q" {
                query = args[i + 1].clone();
            } else if arg == "-p" {
                file_path = args[i + 1].clone();
            } else if arg == "--ignore-case" {
                ignore_case = true;
            }
        }
        Ok(Config{ query, file_path, ignore_case})


    }
}

fn find_word(query: &str, line: &str) -> usize {
    for (i, word) in line.split_whitespace().enumerate() {
        if word.contains(&query) {
            return  i as usize
        }
    }
    0
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        ignore_case_search(&config.query, &contents)
    } else {
        normal_search(&config.query, &contents)
    };

    for line in results {
        let position = find_word(&config.query, &line);
        for (i, word) in line.split_whitespace().enumerate() {
            if i == position {
                print!("{}{}{}{} ", RED, UNDERLINE, word, RESET);
            } else {
                print!("{} ", word);
            }
        }
        println!();
    }
    Ok(())
}
