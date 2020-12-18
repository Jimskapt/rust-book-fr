pub struct Billet {
    contenu: String,
}

pub struct BrouillonDeBillet {
    contenu: String,
}

impl Billet {
    pub fn new() -> BrouillonDeBillet {
        BrouillonDeBillet {
            contenu: String::new(),
        }
    }

    pub fn contenu(&self) -> &str {
        &self.contenu
    }
}

impl BrouillonDeBillet {
    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }

    pub fn demander_relecture(self) -> BilletEnRelecture {
        BilletEnRelecture {
            contenu: self.contenu,
        }
    }
}

pub struct BilletEnRelecture {
    contenu: String,
}

impl BilletEnRelecture {
    pub fn approuver(self) -> Billet {
        Billet {
            contenu: self.contenu,
        }
    }
}
