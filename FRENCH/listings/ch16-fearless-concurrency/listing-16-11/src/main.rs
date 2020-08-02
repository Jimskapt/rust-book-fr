use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ANCHOR: here
    // -- partie masquée ici --

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let valeurs = vec![
            String::from("salutations"),
            String::from("à partir"),
            String::from("de la"),
            String::from("nouvelle tâche"),
        ];

        for valeur in valeurs {
            tx1.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let valeurs = vec![
            String::from("encore plus"),
            String::from("de messages"),
            String::from("pour"),
            String::from("vous"),
        ];

        for valeur in valeurs {
            tx.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recu in rx {
        println!("On a reçu : {}", recu);
    }

    // -- partie masquée ici --
    // ANCHOR_END: here
}
