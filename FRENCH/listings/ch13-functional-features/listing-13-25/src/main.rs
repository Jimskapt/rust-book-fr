use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: here
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    // -- partie masquée ici --
    // ANCHOR_END: here

    if let Err(e) = minigrep::run(config) {
        eprintln!("Erreur applicative : {}", e);

        process::exit(1);
    }
    // ANCHOR: here
}
// ANCHOR_END: here
