use std::fs::File;

fn main() {
    let resultat_fichier_salut = File::open("hello.txt");

    let resultat_fichier_salut = match resultat_fichier_salut {
        Ok(fichier) => fichier,
        Err(erreur) => panic!("Erreur d'ouverture du fichier : {:?}", erreur),
    };
}
