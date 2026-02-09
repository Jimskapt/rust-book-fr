use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    loop {
        // -- partie masquée ici --

        // ANCHOR_END: here
        println!("Veuillez saisir un nombre.");

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de la saisie");

        // ANCHOR: here
        let supposition: i32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        if supposition < 1 || supposition > 100 {
            println!("Le nombre secret est entre 1 et 100.");
            continue;
        }

        match supposition.cmp(&nombre_secret) {
            // -- partie masquée ici --
            // ANCHOR_END: here
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Gagné !");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
