use blog::Billet;

fn main() {
    let mut billet = Billet::new();

    billet.ajouter_texte("J'ai mangé une salade au déjeuner aujourd'hui");

    let billet = billet.demander_relecture();

    let billet = billet.approuver();

    assert_eq!("J'ai mangé une salade au déjeuner aujourd'hui", billet.contenu());
}
