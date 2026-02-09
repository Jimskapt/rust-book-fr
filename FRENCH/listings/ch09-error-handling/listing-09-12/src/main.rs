use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let fichier_salut = File::open("hello.txt")?;

    Ok(())
}
