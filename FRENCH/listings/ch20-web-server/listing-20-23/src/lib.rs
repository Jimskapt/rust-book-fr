use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// ANCHOR: here
pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Message>,
}

// -- partie masquée ici --

// ANCHOR_END: here
type Mission = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NouvelleMission(Mission),
    Extinction,
}

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

    // ANCHOR: here
    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mission = Box::new(f);

        self.envoi.send(Message::NouvelleMission(mission)).unwrap();
    }
}

// -- partie masquée ici --

// ANCHOR_END: here
impl Drop for GroupeTaches {
    fn drop(&mut self) {
        for operateur in &mut self.operateurs {
            println!("Arrêt de l'opérateur {}", operateur.id);

            if let Some(tache) = operateur.tache.take() {
                tache.join().unwrap();
            }
        }
    }
}

struct Operateur {
    id: usize,
    tache: Option<thread::JoinHandle<()>>,
}

// ANCHOR: here
impl Operateur {
    fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Message>>>) -> Operateur {
        let tache = thread::spawn(move || loop {
            let message = reception.lock().unwrap().recv().unwrap();

            match message {
                Message::NouvelleMission(mission) => {
                    println!("L'opérateur {} a reçu une mission ; il l'exécute.", id);

                    mission();
                }
                Message::Extinction => {
                    println!("L'opérateur {} a reçu l'instruction d'arrêt.", id);

                    break;
                }
            }
        });

        Operateur {
            id,
            tache: Some(tache),
        }
    }
}
// ANCHOR_END: here
