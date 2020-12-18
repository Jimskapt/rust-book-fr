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

    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }

    pub fn contenu(&self) -> &str {
        ""
    }

    // ANCHOR: here
    pub fn demander_relecture(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.demander_relecture())
        }
    }
}

trait Etat {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat>;
}

struct Brouillon {}

impl Etat for Brouillon {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(EnRelecture {})
    }
}

struct EnRelecture {}

impl Etat for EnRelecture {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }
}
// ANCHOR_END: here

fn main() {}
