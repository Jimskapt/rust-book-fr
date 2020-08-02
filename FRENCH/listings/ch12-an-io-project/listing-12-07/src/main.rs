use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // ANCHOR_END: here

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    let contenu = fs::read_to_string(config.nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    println!("Dans le texte :\n{}", contenu);
    // ANCHOR: here

    // -- partie masquée ici --
}

// -- partie masquée ici --

// ANCHOR_END: here
struct Config {
    recherche: String,
    nom_fichier: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Config {
        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        Config { recherche, nom_fichier }
    }
}
// ANCHOR_END: here
