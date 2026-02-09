// ANCHOR: here
pub trait Resumable {
    fn resumer(&self) -> String {
        String::from("(En savoir plus ...)")
    }
}
// ANCHOR_END: here

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {}

pub struct PublicationSociale {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub republication: bool,
}

impl Resumable for PublicationSociale {
    fn resumer(&self) -> String {
        format!("{}: {}", self.nom_utilisateur, self.contenu)
    }
}
