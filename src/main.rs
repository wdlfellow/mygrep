use std::env;
use std::error::Error;
use std::fs;
use std::process;

use mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1)
    });

    if let Err(e) = mygrep::run(config){
        println!("Error: {}", e);
        process::exit(1);
    }

}

