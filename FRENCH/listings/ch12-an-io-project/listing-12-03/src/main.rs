use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let recherche = &args[1];
    let nom_fichier = &args[2];

    println!("On recherche : {}", recherche);
    println!("Dans le fichier : {}", nom_fichier);
}
