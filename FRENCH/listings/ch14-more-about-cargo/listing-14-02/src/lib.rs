//! # Ma crate
//!
//! `ma_crate` est un regroupement d'utilitaires pour rendre plus pratique
//! certains calculs.

/// Ajoute 1 au nombre donné.
// -- partie masquée ici --
// ANCHOR_END: here
///
/// # Exemples
///
/// ```
/// let argument = 5;
/// let reponse = ma_crate::ajouter_un(argument);
///
/// assert_eq!(6, reponse);
/// ```
pub fn ajouter_un(x: i32) -> i32 {
    x + 1
}
