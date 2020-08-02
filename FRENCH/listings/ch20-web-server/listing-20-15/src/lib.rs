// ANCHOR: here
use std::thread;

pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
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

        let mut operateurs = Vec::with_capacity(taille);

        for id in 0..taille {
            operateurs.push(Operateur::new(id));
        }

        GroupeTaches { operateurs }
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

struct Operateur {
    id: usize,
    tache: thread::JoinHandle<()>,
}

impl Operateur {
    fn new(id: usize) -> Operateur {
        let tache = thread::spawn(|| {});

        Operateur { id, tache }
    }
}
// ANCHOR_END: here

fn main() {}
