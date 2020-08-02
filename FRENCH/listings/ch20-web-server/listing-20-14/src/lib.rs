// ANCHOR: here
use std::thread;

pub struct GroupeTaches {
    taches: Vec<thread::JoinHandle<()>>,
}

impl GroupeTaches {
    // -- partie masquée ici --
    // ANCHOR_END: here
    /// Crée un nouveau GroupeTaches.
    ///
    /// La taille est le nom de tâches présentes dans le groupe.
    ///
    /// # Panics
    ///
    /// La fonction `new` devrait paniquer si la taille vaut zéro.
    // ANCHOR: here
    pub fn new(taille: usize) -> GroupeTaches {
        assert!(taille > 0);

        let mut taches = Vec::with_capacity(taille);

        for _ in 0..taille {
            // on crée quelques tâches ici et on les stocke dans le vecteur
        }

        GroupeTaches { taches }
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
