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

impl Drop for GroupeTaches {
    fn drop(&mut self) {
        for operateur in &mut self.operateurs {
            println!("Arrêt de l'opérateur {}", operateur.id);

            operateur.tache.join().unwrap();
        }
    }
}

struct Operateur {
    id: usize,
    tache: Option<thread::JoinHandle<()>>,
}

// ANCHOR: here
impl Operateur {
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Mission>>>) -> Operateur {
        // -- partie masquée ici --

        // ANCHOR_END: here
        let tache = thread::spawn(move || loop {
            let mission = reception.lock().unwrap().recv().unwrap();

            println!("L'opérateur {} a reçu une mission ; il l'exécute.", id);

            mission();
        });

        // ANCHOR: here
        Operateur {
            id,
            tache: Some(tache),
        }
    }
}
// ANCHOR_END: here
