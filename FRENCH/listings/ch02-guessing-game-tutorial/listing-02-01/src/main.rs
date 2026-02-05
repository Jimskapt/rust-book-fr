// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Devinez le nombre !");

    println!("Veuillez entrer un nombre.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut supposition = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut supposition)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Échec de la lecture de l'entrée utilisateur");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Votre nombre : {supposition}");
    // ANCHOR_END: print_guess
}
// ANCHOR: all
