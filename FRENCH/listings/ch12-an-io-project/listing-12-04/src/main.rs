// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // -- partie masquée ici --
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let recherche = &args[1];
    let nom_fichier = &args[2];

    println!("On recherche : {}", recherche);
    // ANCHOR: here
    println!("Dans le fichier : {}", nom_fichier);

    let contenu = fs::read_to_string(nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    println!("Dans le texte :\n{}", contenu);
}
// ANCHOR_END: here
