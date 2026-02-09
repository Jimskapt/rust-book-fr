use std::fs::File;

fn main() {
    let fichier_salut = File::open("hello.txt")?;
}
