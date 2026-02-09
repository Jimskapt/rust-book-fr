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
    match piece {
        PieceUs::Quarter(etat) => println!("Il s'agit d'un quarter de l'État de {etat:?} !"),
        _ => compteur += 1,
    }
    // ANCHOR_END: here
}
