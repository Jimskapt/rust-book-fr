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

fn main() {
    println!("Un bébé chien s'appelle un {}", Chien::nom_bebe());
}
