use blog::Billet;

fn main() {
    let mut billet = Billet::new();

    billet.ajouter_texte("J'ai mangé une salade au déjeuner aujourd'hui");
    assert_eq!("", billet.contenu());

    billet.demander_relecture();
    assert_eq!("", billet.contenu());

    billet.approuver();
    assert_eq!("J'ai mangé une salade au déjeuner aujourd'hui", billet.contenu());
}
