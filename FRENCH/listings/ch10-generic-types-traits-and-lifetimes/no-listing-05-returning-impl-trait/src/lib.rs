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
fn retourne_resumable() -> impl Resumable {
    PublicationSociale {
        nom_utilisateur: String::from("jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        republication: false,
    }
}
// ANCHOR_END: here
