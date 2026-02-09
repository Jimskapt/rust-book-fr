pub trait Resumable {
    fn resumer(&self) -> String;
}

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {
    fn resumer(&self) -> String {
        format!("{}, par {} ({})", self.titre, self.auteur, self.lieu)
    }
}

pub struct PublicationSociale {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub republication: bool,
}

impl Resumable for PublicationSociale {
    fn resumer(&self) -> String {
        format!("{} : {}", self.nom_utilisateur, self.contenu)
    }
}

// ANCHOR: here
pub fn notifier(element: &impl Resumable) {
    println!("Flash info ! {}", element.resumer());
}
// ANCHOR_END: here
