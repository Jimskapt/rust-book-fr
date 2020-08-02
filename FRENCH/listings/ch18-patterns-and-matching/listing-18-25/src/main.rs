fn main() {
    let nombres = (2, 4, 8, 16, 32);

    match nombres {
        (.., second, ..) => {
            println!("Voici quelques nombres : {}", second)
        },
    }
}
