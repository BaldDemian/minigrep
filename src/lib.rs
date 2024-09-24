use std::error::Error;
use std::{env, fs};
use regex::Regex;

const ORANGE_START: &str = "\x1b[38;5;214m";
const COLOR_RESET: &str = "\x1b[0m";
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // 因为读取文件时可能发生多种错误，所以返回值是一个trait object
    let contents = fs::read_to_string(config.file_path)?; // 使用?自动传播Err
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
fn search(query: &str, contents: &str) -> Vec<String> { // 因为涉及到修改原始字符串，所以这里直接返回String了
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.replace(query, &format!("{}{}{}", ORANGE_START, query, COLOR_RESET)))
        .collect()
}
fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    let re = Regex::new(&format!(r"(?i){}", regex::escape(&query))).unwrap();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|line| {
            re.replace_all(line, |caps: &regex::Captures| {
                format!("{}{}{}", ORANGE_START, &caps[0], COLOR_RESET)
            })
                .to_string()
        })
        .collect()
}
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // this function will take ownership of args
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        if !ignore_case && args.next().is_some_and(|v| v.eq_ignore_ascii_case("-i")) {
            ignore_case = true;
        }
        Ok(Self { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}