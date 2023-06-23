use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let _args = args.clone();

    //dbg!(args);

    println!("{}", &_args[1]);
    println!("{}", &_args[2]);
}
