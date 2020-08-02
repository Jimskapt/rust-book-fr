use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let manipulateur = thread::spawn(move || {
        println!("Voici un vecteur : {:?}", v);
    });

    manipulateur.join().unwrap();
}
