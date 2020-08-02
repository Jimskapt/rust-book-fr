enum Couleur {
    Rvb(i32, i32, i32),
    Tsv(i32, i32, i32),
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(Couleur),
}

fn main() {
    let msg = Message::ChangerCouleur(Couleur::Tsv(0, 160, 255));

    match msg {
        Message::ChangerCouleur(Couleur::Rvb(r, v, b)) => println!(
            "Changement des taux de rouge à {}, de vert à {}, et de bleu à {}",
            r, v, b
        ),
        Message::ChangerCouleur(Couleur::Tsv(t, s, v)) => println!(
            "Changement des taux de teinte à {}, de saturation à {}, et de valeur à {}",
            t, s, v
        ),
        _ => (),
    }
}
