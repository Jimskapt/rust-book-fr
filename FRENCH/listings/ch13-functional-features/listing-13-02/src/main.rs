use std::thread;
use std::time::Duration;

fn simuler_gros_calcul(intensite: u32) -> u32 {
    println!("calcul très lent ...");
    thread::sleep(Duration::from_secs(2));
    intensite
}

fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {}

// ANCHOR: here
fn main() {
    let valeur_utilisateur_simule = 10;
    let nombre_aleatoire_simule = 7;

    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);
}
// ANCHOR_END: here
