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

    pub fn demander_relecture(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.demander_relecture())
        }
    }

    // ANCHOR: here
    pub fn approuver(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.approuver())
        }
    }
}

trait Etat {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat>;
    fn approuver(self: Box<Self>) -> Box<dyn Etat>;
}

struct Brouillon {}

impl Etat for Brouillon {
    // -- partie masquée ici --
    // ANCHOR_END: here
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(EnRelecture {})
    }

    // ANCHOR: here
    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }
}

struct EnRelecture {}

impl Etat for EnRelecture {
    // -- partie masquée ici --
    // ANCHOR_END: here
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    // ANCHOR: here
    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(Publier {})
    }
}

struct Publier {}

impl Etat for Publier {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }
}
// ANCHOR_END: here
