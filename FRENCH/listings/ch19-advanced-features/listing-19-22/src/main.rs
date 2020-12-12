// ANCHOR: here
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
// ANCHOR_END: here

fn main() {}
