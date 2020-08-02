struct PointeurPerso {
    donnee: String,
}

impl Drop for PointeurPerso {
    fn drop(&mut self) {
        println!("Nettoyage d'un PointeurPerso avec la donnée `{}` !", self.donnee);
    }
}

// ANCHOR: here
fn main() {
    let c = PointeurPerso {
        donnee: String::from("des trucs"),
    };
    println!("PointeurPerso créé.");
    c.drop();
    println!("PointeurPerso libéré avant la fin du main.");
}
// ANCHOR_END: here
