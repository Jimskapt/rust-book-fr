use std::thread;
use std::time::Duration;

fn simuler_gros_calcul(intensite: u32) -> u32 {
    println!("calcul très lent ...");
    thread::sleep(Duration::from_secs(2));
    intensite
}

// ANCHOR: here
fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
    if intensite < 25 {
        println!(
            "Aujourd'hui, faire {} pompes !",
            simuler_gros_calcul(intensite)
        );
        println!(
            "Ensuite, faire {} abdominaux !",
            simuler_gros_calcul(intensite)
        );
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
        } else {
            println!(
                "Aujourd'hui, courrez pendant {} minutes !",
                simuler_gros_calcul(intensite)
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
