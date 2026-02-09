use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    println!("Le nombre secret est : {nombre_secret}");

    loop {
        println!("Veuillez entrer un nombre.");

        let mut supposition = String::new();

        // ANCHOR: here
        // -- partie masquée ici --

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        // ANCHOR: ch19
        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Votre nombre : {supposition}");

        // -- partie masquée ici --
        // ANCHOR_END: here

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
