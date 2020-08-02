use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = interpreter_config(&args);

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    let contenu = fs::read_to_string(config.nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    // -- partie masquée ici --
    // ANCHOR_END: here

    println!("Dans le texte :\n{}", contenu);
    // ANCHOR: here
}

struct Config {
    recherche: String,
    nom_fichier: String,
}

fn interpreter_config(args: &[String]) -> Config {
    let recherche = args[1].clone();
    let nom_fichier = args[2].clone();

    Config { recherche, nom_fichier }
}
// ANCHOR_END: here
