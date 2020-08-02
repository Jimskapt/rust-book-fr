use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    let contenu = fs::read_to_string(config.nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    println!("Dans le texte :\n{}", contenu);
}

struct Config {
    recherche: String,
    nom_fichier: String,
}

impl Config {
    // ANCHOR: here
    // -- partie masquée ici --
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("il n'y a pas assez d'arguments");
        }
        // -- partie masquée ici --
        // ANCHOR_END: here

        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        Config { recherche, nom_fichier }
    }
}
