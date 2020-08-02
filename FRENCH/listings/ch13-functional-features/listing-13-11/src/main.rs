use std::thread;
use std::time::Duration;

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32
{
    fn new(calcul: T) -> Cache<T> {
        Cache {
            calcul,
            valeur: None,
        }
    }

    fn valeur(&mut self, arg: u32) -> u32 {
        match self.valeur {
            Some(v) => v,
            None => {
                let v = (self.calcul)(arg);
                self.valeur = Some(v);
                v
            },
        }
    }
}

// ANCHOR: here
fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
    let mut resultat_lent = Cache::new(|nombre| {
        println!("calcul très lent ...");
        thread::sleep(Duration::from_secs(2));
        nombre
    });

    if intensite < 25 {
        println!("Aujourd'hui, faire {} pompes !", resultat_lent.valeur(intensite));
        println!("Ensuite, faire {} abdominaux !", resultat_lent.valeur(intensite));
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
        } else {
            println!(
                "Aujourd'hui, courrez pendant {} minutes !",
                resultat_lent.valeur(intensite)
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
