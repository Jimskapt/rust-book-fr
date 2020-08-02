use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let manipulateur = thread::spawn(|| {
        println!("Voici un vecteur : {:?}", v);
    });

    drop(v); // oh, non !

    manipulateur.join().unwrap();
}
