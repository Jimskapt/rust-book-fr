#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

// ANCHOR: here
impl Rectangle {
    fn carre(cote: u32) -> Self {
        Self {
            largeur: cote,
            hauteur: cote
        }
    }
}
// ANCHOR_END: here

fn main() {
    let mon_carre = Rectangle::carre(3);
}
