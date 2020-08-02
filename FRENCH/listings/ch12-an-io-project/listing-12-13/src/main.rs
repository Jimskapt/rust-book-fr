use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    if let Err(e) = run(config) {
        println!("Erreur applicative : {}", e);

        process::exit(1);
    }
}
