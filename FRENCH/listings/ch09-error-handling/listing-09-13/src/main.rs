use rand::Rng;
use std::cmp::Ordering;
use std::io;

// ANCHOR: here
pub struct Supposition {
    valeur: i32,
}

impl Supposition {
    pub fn new(valeur: i32) -> Supposition {
        if valeur < 1 || valeur > 100 {
            panic!("La valeur de Supposition doit ĉtre comprise entre 1 et 100, nous avons {}.", valeur);
        }

        Supposition { valeur }
    }

    pub fn valeur(&self) -> i32 {
        self.valeur
    }
}
// ANCHOR_END: here

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Veuillez entrer un nombre.");

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let supposition: i32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        let supposition = Supposition::new(supposition);

        match supposition.valeur().cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
