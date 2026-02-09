// ANCHOR: here
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // -- partie masquée ici --
    // ANCHOR_END: here
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    println!("Le nombre secret est : {}", nombre_secret);

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");
    // ANCHOR: here

    println!("Votre nombre : {}", supposition);

    match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!("C'est plus !"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => println!("Vous avez gagné !"),
    }
}
// ANCHOR_END: here
