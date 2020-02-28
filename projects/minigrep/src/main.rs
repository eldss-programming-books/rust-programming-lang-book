use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Get command line arguments and put into a Vector
    // To accept non Unicode args, use std::env::args_os
    // let args: Vec<String> = env::args().collect();

    // Store vars
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Run the program
    // If error occurs print message and stop
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
