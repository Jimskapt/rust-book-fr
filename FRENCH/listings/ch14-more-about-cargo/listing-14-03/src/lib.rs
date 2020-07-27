// ANCHOR: here
//! # Art
//!
//! Une bibliothèque pour modéliser des concepts artistiques.

pub mod types {
    /// Les couleurs primaires du modèle RJB.
    pub enum CouleurPrimaire {
        Rouge,
        Jaune,
        Bleu,
    }

    /// Les couleurs secondaires du modèle RJB.
    pub enum CouleurSecondaire {
        Orange,
        Vert,
        Violet,
    }
}

pub mod utilitaires {
    use crate::types::*;

    /// Combine deux couleurs primaires dans les mêmes quantités pour
    /// créer une couleur secondaire.
    pub fn mixer(c1: CouleurPrimaire, c2: CouleurPrimaire) -> CouleurSecondaire {
        // -- partie masquée ici --
        // ANCHOR_END: here
        CouleurSecondaire::Orange
        // ANCHOR: here
    }
}
// ANCHOR_END: here

fn main() {}
