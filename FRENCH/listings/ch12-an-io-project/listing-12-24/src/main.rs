use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Erreur applicative : {}", e);

        process::exit(1);
    }
}
// ANCHOR_END: here
