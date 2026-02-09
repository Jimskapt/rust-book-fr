use std::fs::File;

fn main() {
    let fichier_salut = File::open("hello.txt")
        .expect("Échec à l'ouverture de hello.txt");
}
