struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle {
        largeur: 30,
        hauteur: 50
    };

    println!("rect1 est {rect1}");
}
