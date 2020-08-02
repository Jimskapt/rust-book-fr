use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Mission>,
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

impl GroupeTaches {
    /// Crée un nouveau GroupeTaches.
    ///
    /// La taille est le nom de tâches présentes dans le groupe.
    ///
    /// # Panics
    ///
    /// La fonction `new` devrait paniquer si la taille vaut zéro.
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

    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mission = Box::new(f);

        self.envoi.send(mission).unwrap();
    }
}

struct Operateur {
    id: usize,
    tache: thread::JoinHandle<()>,
}
// ANCHOR: here
// -- partie masquée ici --

impl Operateur {
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Mission>>>) -> Operateur {
        let tache = thread::spawn(move || {
            while let Ok(mission) = reception.lock().unwrap().recv() {
                println!("L'opérateur {} a obtenu une mission ; il l'exécute.", id);

                mission();
            }
        });

        Operateur { id, tache }
    }
}
// ANCHOR_END: here
