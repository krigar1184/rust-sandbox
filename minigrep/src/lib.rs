use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;



pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(&config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| {
            line.to_lowercase().contains(&query)
        })
        .collect()
}


pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}


impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a file name"),
        };

        let mut options = Vec::new();

        for arg in args {
            if arg.starts_with("-") || arg.starts_with("--") {
                options.push(arg);
            }
        }

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if options.iter().any(|ref x| x == &&String::from("-i") || x == &&String::from("--case-insensitive")) {
            case_sensitive = false;
        }

        Ok(Config { query, filename, case_sensitive })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_insensitive() {
        let query = "RuSt";
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
