use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: ch13
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    // -- partie masquée ici --
    // ANCHOR_END: ch13

    if let Err(e) = minigrep::run(config) {
        eprintln!("Erreur applicative : {}", e);

        process::exit(1);
    }
    // ANCHOR: ch13
}
// ANCHOR_END: ch13
