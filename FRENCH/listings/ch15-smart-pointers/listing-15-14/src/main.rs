struct PointeurPerso {
    donnee: String,
}

impl Drop for PointeurPerso {
    fn drop(&mut self) {
        println!("Nettoyage d'un PointeurPerso avec la donnée `{}` !", self.donnee);
    }
}

fn main() {
    let c = PointeurPerso {
        donnee: String::from("des trucs"),
    };
    let d = PointeurPerso {
        donnee: String::from("d'autres trucs"),
    };
    println!("PointeurPersos créés.");
}
