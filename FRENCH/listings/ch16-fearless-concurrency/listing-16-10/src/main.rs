use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let valeurs = vec![
            String::from("salutations"),
            String::from("à partir"),
            String::from("de la"),
            String::from("nouvelle tâche"),
        ];

        for valeur in valeurs {
            tx.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recu in rx {
        println!("On a reçu : {}", recu);
    }
}
