// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Devinez le nombre !");

    // ANCHOR: ch07-04
    let nombre_secret = rand::thread_rng().gen_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Le nombre secret est : {nombre_secret}");

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

        println!("Votre nombre : {supposition}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
