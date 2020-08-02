enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangerCouleur(0, 160, 255);

    match msg {
        Message::Quitter => {
            println!("La variante Quitter n'a pas de données à déstructurer.")
        }
        Message::Deplacer { x, y } => {
            println!(
                "Déplacement de {} sur l'axe x et de {} sur l'axe y",
                x, y
            );
        }
        Message::Ecrire(text) => println!("Message textuel : {}", text),
        Message::ChangerCouleur(r, g, b) => println!(
            "Changement des taux de rouge à {}, de vert à {}, et de bleu à {}",
            r, g, b
        ),
    }
}
