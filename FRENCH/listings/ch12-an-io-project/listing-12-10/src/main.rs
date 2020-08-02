use std::env;
use std::fs;
// ANCHOR: here
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    // -- partie masquée ici --
    // ANCHOR_END: here

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("il n'y a pas assez d'arguments");
        }

        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        Ok(Config { recherche, nom_fichier })
    }
}
