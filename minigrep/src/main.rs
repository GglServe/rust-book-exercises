use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let case_insensitive = env::var("CASE_IN").is_ok();
    let args = Config::new(args, case_insensitive).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
