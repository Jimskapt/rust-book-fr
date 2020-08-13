use std::sync::Mutex;
use std::thread;

fn main() {
    let compteur = Mutex::new(0);
    let mut manipulateurs = vec![];

    for _ in 0..10 {
        let manipulateur = thread::spawn(move || {
            let mut nombre = compteur.lock().unwrap();

            *nombre += 1;
        });
        manipulateurs.push(manipulateur);
    }

    for manipulateur in manipulateurs {
        manipulateur.join().unwrap();
    }

    println!("Resultat : {}", *compteur.lock().unwrap());
}
