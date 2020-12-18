pub struct Billet {
    etat: Option<Box<dyn Etat>>,
    contenu: String,
}

impl Billet {
    pub fn new() -> Billet {
        Billet {
            etat: Some(Box::new(Brouillon {})),
            contenu: String::new(),
        }
    }
}

trait Etat {}

struct Brouillon {}

impl Etat for Brouillon {}
