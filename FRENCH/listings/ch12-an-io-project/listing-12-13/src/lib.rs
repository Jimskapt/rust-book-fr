// ANCHOR: here
use std::error::Error;
use std::fs;

pub struct Config {
    pub recherche: String,
    pub nom_fichier: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // -- partie masquée ici --
        // ANCHOR_END: here
        if args.len() < 3 {
            return Err("il n'y a pas assez d'arguments");
        }

        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        Ok(Config { recherche, nom_fichier })
        // ANCHOR: here
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // -- partie masquée ici --
    // ANCHOR_END: here
    let contenu = fs::read_to_string(config.nom_fichier)?;

    println!("Dans le texte :\n{}", contenu);

    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
