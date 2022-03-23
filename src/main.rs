use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let arg: Vec<String> = env::args().collect();

    let config = Config::new(&arg).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
