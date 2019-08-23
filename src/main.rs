use std::error::Error;
use std::{env, fs, process};

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> indica un type che implementa il trait (padre) Error, lo spiega al capitolo 17
    let contents = fs::read_to_string(config.filename)?; // ? = return Err(message)
    println!("With text:\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Cannot find file -> {}", err);
        process::exit(-1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
