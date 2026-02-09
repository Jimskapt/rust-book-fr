// ANCHOR: here
mod salle_a_manger {
    pub mod accueil {
        fn ajouter_a_la_liste_attente() {}
    }
}

// -- partie masquée ici --
// ANCHOR_END: here
pub fn manger_au_restaurant() {
    // Chemin absolu
    crate::salle_a_manger::accueil::ajouter_a_la_liste_attente();

    // Chemin relatif
    salle_a_manger::accueil::ajouter_a_la_liste_attente();
}
