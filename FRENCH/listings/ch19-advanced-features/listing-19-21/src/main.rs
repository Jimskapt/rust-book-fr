trait Animal {
    fn nom_bebe() -> String;
}

struct Chien;

impl Chien {
    fn nom_bebe() -> String {
        String::from("Spot")
    }
}

impl Animal for Chien {
    fn nom_bebe() -> String {
        String::from("chiot")
    }
}

// ANCHOR: here
fn main() {
    println!("Un bébé chien s'appelle un {}", <Chien as Animal>::nom_bebe());
}
// ANCHOR_END: here
