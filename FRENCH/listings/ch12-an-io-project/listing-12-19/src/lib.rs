use std::error::Error;
use std::fs;

pub struct Config {
    pub recherche: String,
    pub nom_fichier: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("il n'y a pas assez d'arguments");
        }

        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        Ok(Config { recherche, nom_fichier })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenu = fs::read_to_string(config.nom_fichier)?;

    Ok(())
}

// ANCHOR: here
// ANCHOR: ch13
pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let mut resultats = Vec::new();

    for ligne in contenu.lines() {
        if ligne.contains(recherche) {
            resultats.push(ligne);
        }
    }

    resultats
}
// ANCHOR_END: ch13
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_resultat() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.";

        assert_eq!(vec!["sécurité, rapidité, productivité."], rechercher(recherche, contenu));
    }
}
