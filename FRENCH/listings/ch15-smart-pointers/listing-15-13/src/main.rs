use std::ops::Deref;

impl<T> Deref for MaBoite<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MaBoite<T>(T);

impl<T> MaBoite<T> {
    fn new(x: T) -> MaBoite<T> {
        MaBoite(x)
    }
}

fn saluer(nom: &str) {
    println!("Salutations, {} !", nom);
}

// ANCHOR: here
fn main() {
    let m = MaBoite::new(String::from("Rust"));
    saluer(&(*m)[..]);
}
// ANCHOR_END: here
