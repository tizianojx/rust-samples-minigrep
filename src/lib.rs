use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> indica un type che implementa il trait (padre) Error, lo spiega al capitolo 17
    let contents = fs::read_to_string(config.filename)?; // ? = return Err(message)
    println!("With text:\n{}", contents);
    Ok(())
}