mod salle_a_manger {
    pub mod accueil {
        pub fn ajouter_a_la_liste_attente() {}
    }
}

use crate::salle_a_manger::accueil::ajouter_a_la_liste_attente;

pub fn manger_au_restaurant() {
    ajouter_a_la_liste_attente();
}
