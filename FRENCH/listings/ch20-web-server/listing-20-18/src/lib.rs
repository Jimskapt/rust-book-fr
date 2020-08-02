use std::sync::mpsc;
use std::thread;
// ANCHOR: here
use std::sync::Arc;
use std::sync::Mutex;
// -- partie masquée ici --

// ANCHOR_END: here
pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Mission>,
}

struct Mission;

// ANCHOR: here
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

        let reception = Arc::new(Mutex::new(reception));

        let mut operateurs = Vec::with_capacity(taille);

        for id in 0..taille {
            operateurs.push(Operateur::new(id, Arc::clone(&reception)));
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

// -- partie masquée ici --

// ANCHOR_END: here
struct Operateur {
    id: usize,
    tache: thread::JoinHandle<()>,
}

// ANCHOR: here
impl Operateur {
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Mission>>>) -> Operateur {
        // -- partie masquée ici --
        // ANCHOR_END: here
        let tache = thread::spawn(|| {
            reception;
        });

        Operateur { id, tache }
        // ANCHOR: here
    }
}
// ANCHOR_END: here

fn main() {}
