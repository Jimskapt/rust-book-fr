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

fn main() {
    let piece = PieceUs::Penny;
    // ANCHOR: here
    let mut compteur = 0;
    if let PieceUs::Quarter(etat) = piece {
        println!("Il s'agit d'un quarter de l'État de {etat:?} !");
    } else {
        compteur += 1;
    }
    // ANCHOR_END: here
}
