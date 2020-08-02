fn afficher_coordonnees(&(x, y): &(i32, i32)) {
    println!("Coordonnées actuelles : ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    afficher_coordonnees(&point);
}
