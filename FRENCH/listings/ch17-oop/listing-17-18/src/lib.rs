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

    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }

    pub fn contenu(&self) -> &str {
        self.etat.as_ref().unwrap().contenu(self)
    }

    pub fn demander_relecture(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.demander_relecture())
        }
    }

    pub fn approuver(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.approuver())
        }
    }
}

// ANCHOR: here
trait Etat {
    // -- partie masquée ici --
    // ANCHOR_END: here
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat>;
    fn approuver(self: Box<Self>) -> Box<dyn Etat>;

    // ANCHOR: here
    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        ""
    }
}

// -- partie masquée ici --
// ANCHOR_END: here

struct Brouillon {}

impl Etat for Brouillon {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(EnRelecture {})
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }
}

struct EnRelecture {}

impl Etat for EnRelecture {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(Publier {})
    }
}

// ANCHOR: here
struct Publier {}

impl Etat for Publier {
    // -- partie masquée ici --
    // ANCHOR_END: here
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    // ANCHOR: here
    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        &billet.contenu
    }
}
// ANCHOR_END: here
