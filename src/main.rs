use std::env;
use std::process;
use miniGrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments : {err}");
        process::exit(1)
    });

    if let Err(e) = miniGrep::run(config) {
        println!("application error : {e}");
        process::exit(1)
    }
}



