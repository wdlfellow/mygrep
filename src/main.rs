use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let (query, path) = parse_config(&args);

    fs::read_to_string(path).expect("the file does not exist!!!");

}

fn parse_config(args: &[String]) -> (&str, &str){
    let query = &args[1];
    let path = &args[2];

    (query, path)
}