
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! 3 are required!");
        } else if args.len() > 3 {
            return Err("Too many arguments! 3 are required!");
        } else {
            Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            })
        }
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n---- BEGIN ----\n{}\n---- END ----", contents);
    Ok(())
}

