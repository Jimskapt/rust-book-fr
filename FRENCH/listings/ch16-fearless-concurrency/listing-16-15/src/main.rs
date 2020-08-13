use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let compteur = Arc::new(Mutex::new(0));
    let mut manipulateurs = vec![];

    for _ in 0..10 {
        let compteur = Arc::clone(&compteur);
        let manipulateur = thread::spawn(move || {
            let mut nombre = compteur.lock().unwrap();

            *nombre += 1;
        });
        manipulateurs.push(manipulateur);
    }

    for manipulateur in manipulateurs {
        manipulateur.join().unwrap();
    }

    println!("Résultat : {}", *compteur.lock().unwrap());
}
