#[derive(Debug)]
enum EtatUs {
    Alabama,
    Alaska,
    // -- partie masquée ici --
}

enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs),
}

// ANCHOR: here
fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => 1,
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter(etat) => {
            println!("Il s'agit d'un quarter de l'État de {etat:?} !");
            25
        },
    }
}
// ANCHOR_END: here

fn main() {
    valeur_en_centimes(PieceUs::Quarter(EtatUs::Alaska));
}
