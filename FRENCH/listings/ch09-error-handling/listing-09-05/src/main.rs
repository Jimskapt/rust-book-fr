use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let  resultat_fichier_salut= File::open("hello.txt");

    let resultat_fichier_salut = match resultat_fichier_salut {
        Ok(fichier) => fichier,
        Err(erreur) => match erreur.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Erreur de création du fichier : {:?}", e),
            },
            autre_erreur => {
                panic!("Erreur d'ouverture du fichier : {:?}", autre_erreur)
            }
        },
    };
}
