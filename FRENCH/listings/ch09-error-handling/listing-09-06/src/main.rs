// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let resultat_fichier_pseudo = File::open("hello.txt");

    let mut resultat_fichier_pseudo = match resultat_fichier_pseudo {
        Ok(fichier) => fichier,
        Err(e) => return Err(e),
    };

    let mut pseudo = String::new();

    match pseudo.read_to_string(&mut pseudo) {
        Ok(_) => Ok(pseudo),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let pseudo =
        lire_pseudo_depuis_fichier().expect("Échec de lecture du pseudo");
}
