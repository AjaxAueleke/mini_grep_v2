use std::env;
use std::process;
use Rust_Practice::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = Rust_Practice::run(config) {
        process::exit(1);
    }
   
}

