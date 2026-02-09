#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn peut_contenir(&self, other: &Rectangle) -> bool {
        self.largeur > other.largeur && self.hauteur > other.hauteur
    }
}
