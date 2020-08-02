// ANCHOR: here
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // -- partie masquée ici --
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    // ANCHOR: here
    if let Err(e) = minigrep::run(config) {
        // -- partie masquée ici --
        // ANCHOR_END: here
        println!("Erreur applicative : {}", e);

        process::exit(1);
        // ANCHOR: here
    }
}
// ANCHOR_END: here
