use std::thread;
// ANCHOR: here
// -- partie masquée ici --
use std::sync::mpsc;

pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Mission>,
}

struct Mission;

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

        let (envoi, reception) = mpsc::channel();

        let mut operateurs = Vec::with_capacity(taille);

        for id in 0..taille {
            operateurs.push(Operateur::new(id));
        }

        GroupeTaches { operateurs, envoi }
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

fn main() {}
