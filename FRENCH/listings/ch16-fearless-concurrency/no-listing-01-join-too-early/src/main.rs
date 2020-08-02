use std::thread;
use std::time::Duration;

fn main() {
    let manipulateur = thread::spawn(|| {
        for i in 1..10 {
            println!("Bonjour n°{} à partir de la nouvelle tâche !", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    manipulateur.join().unwrap();

    for i in 1..5 {
        println!("Bonjour n°{} à partir de la tâche principale !", i);
        thread::sleep(Duration::from_millis(1));
    }
}
