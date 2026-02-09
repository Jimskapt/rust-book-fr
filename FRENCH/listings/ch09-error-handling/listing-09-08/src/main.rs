// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let mut pseudo = String::new();

    File::open("hello.txt")?.read_to_string(&mut pseudo)?;

    Ok(pseudo)
}
// ANCHOR_END: here

fn main() {
    let pseudo =
        lire_pseudo_depuis_fichier().expect("Échec de lecture du pseudo");
}
