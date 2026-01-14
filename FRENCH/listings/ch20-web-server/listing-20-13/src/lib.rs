pub struct GroupeTaches;

// ANCHOR: here
impl GroupeTaches {
    /// Crée un nouveau GroupeTaches.
    ///
    /// La taille est le nombre de tâches présentes dans le groupe.
    ///
    /// # Panics
    ///
    /// La fonction `new` devrait paniquer si la taille vaut zéro.
    pub fn new(taille: usize) -> GroupeTaches {
        assert!(taille > 0);

        GroupeTaches
    }

    // -- partie masquée ici --
    // ANCHOR_END: here

    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}
// ANCHOR_END: here
