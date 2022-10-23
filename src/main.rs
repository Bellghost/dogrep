use dogrep::{run, Config};
use std::{env, process};
fn main() {
    // Parsing config
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // Run...
    run(config);
}