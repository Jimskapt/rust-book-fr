pub struct Billet {
    etat: Option<Box<dyn Etat>>,
    contenu: String,
}

// ANCHOR: here
impl Billet {
    // -- partie masquée ici --
    // ANCHOR_END: here
    pub fn new() -> Billet {
        Billet {
            etat: Some(Box::new(Brouillon {})),
            contenu: String::new(),
        }
    }

    // ANCHOR: here
    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }
}
// ANCHOR_END: here

trait Etat {}

struct Brouillon {}

impl Etat for Brouillon {}

fn main() {}
