// ANCHOR: here
#[derive(Debug)] // pour pouvoir afficher l'État
enum EtatUs {
    Alabama,
    Alaska,
    // -- partie masquée ici --
}

enum Piece {
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs),
}
// ANCHOR_END: here

fn main() {}
