use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // We annotate the args variable to explicitly specify
    // the collection we want to be returned from .collect() method
    // Although we very rarely need to annotate types in Rust,
    // collect is one function we often do need to annotate because
    // Rust isn't able to infer the kind of collection we want.
    // let args: Vec<String> = env::args().collect();

    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // Using unwrap_or_else allows us to define some custom
    // non-panic! error handling. If the Result is an OK value,
    // this method's behavior is similar to unwrap, it returns
    // the inner valus Ok is wrapping. However, if the value is an Err
    // value, this method calls the code in the closure, which is an
    // anonymous functions we define and pass as an argument to
    // unwrap_or_else.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);
        // This prints stderr instead of stdout, easily to
        // be catch
        eprintln!("Problem parsing arguments: {}", err);
        // Exits the program
        process::exit(1);
    });

    // println!("{:?}", args);
    // println!("Searching for {}", query);
    // println!("Searching for {}", config.query);
    // println!("In file {}", filename);
    // println!("In file {}", config.filename);

    // We use if let rather than unwrap_or_else
    // to check whether run returns an Err value
    // and call process::exit(1) if it does.
    // The run function doesn't return a value
    // that we want to unwrap in the same way the
    // Config::new returns the Config instance.
    // Because run returns () in the success case, we
    // onl care about detecting an error, we don't
    // need unwrap_or_else to return the
    // unwrapped value because it would only be ()
    // The bodies of the if let and the unwrap_or_else
    // functions are the same in both cases.
    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    // run(config);
}
