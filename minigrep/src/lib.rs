use std::env;
use std::error::Error;
use std::fs;

// Since query and filename are related together, we can create
// a struct to hold their values
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // fn parse_config(args: &[String]) -> (&str, &str) {
    // fn parse_config(args: &[String]) -> Config {
    // fn new(args: &[String]) -> Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skip first index
        args.next(); // index 0

        // Error handling
        // if args.len() < 3 {
        // Throw exception
        // panic! macro also ends the program immediately
        // panic!("Not enough arguments");
        // return Err("Not enough arguments");
        // }

        // args always take the pwd on where the file is currently
        // being called so we need to start from 1 to avoid this
        // reference
        // let query = &args[1];
        // let query = args[1].clone();
        let query = match args.next() {
            // index 1
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        // let filename = &args[2];
        // let filename = args[2].clone();
        let filename = match args.next() {
            // index 2
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // (query, filename)
        // Config { query, filename }
        // Ok(Config { query, filename })
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> means the function will return a type
// that implements the Error trait, but we don't have to
// specify what particular type the return value will be.
// This gives us flexibility to return error value that may
// be of different types in different error cases.
// The dyn keyword is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read_to_string takes the filename, opens that file and
    // returns a Result<String> of the file's content
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents =
        // fs::read_to_string(config.filename).expect("Something went wrong reading the file");
        // Rather than panic! on an error, the ? will
        // return the error value from the current 
        // function for the caller to handle
        fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // for line in search(&config.query, &contents) {
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// The lifetime parameters specify which argument lifetime
// is connected to teh lifetime of the return value.
// We're telling Rust that the data returned by search will
// live as long as the data passed into the search function in the
// contents argument.
// Because contents is the argument that contains all of our text and we
// want to return the parts of that text that match, we know contents
// is the argument that should be connected to the return value
// using the lifetime syntax.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    //
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

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
