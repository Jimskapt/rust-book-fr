use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let valeur = self.to_string();
        let largeur = valeur.len();
        println!("{}", "*".repeat(largeur + 4));
        println!("*{}*", " ".repeat(largeur + 2));
        println!("* {} *", valeur);
        println!("*{}*", " ".repeat(largeur + 2));
        println!("{}", "*".repeat(largeur + 4));
    }
}

// ANCHOR: here
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
// ANCHOR_END: here

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}
