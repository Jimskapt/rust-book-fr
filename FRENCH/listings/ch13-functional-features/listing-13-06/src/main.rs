use std::thread;
use std::time::Duration;

// ANCHOR: here
fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
    let fermeture_lente = |nombre| {
        println!("calcul très lent ...");
        thread::sleep(Duration::from_secs(2));
        nombre
    };

    if intensite < 25 {
        println!("Aujourd'hui, faire {} pompes !", fermeture_lente(intensite));
        println!("Ensuite, faire {} abdominaux !", fermeture_lente(intensite));
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
        } else {
            println!(
                "Aujourd'hui, courrez pendant {} minutes !",
                fermeture_lente(intensite)
            );
        }
    }
}
// ANCHOR_END: here

fn main() {
    let valeur_utilisateur_simule = 10;
    let nombre_aleatoire_simule = 7;

    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);
}
