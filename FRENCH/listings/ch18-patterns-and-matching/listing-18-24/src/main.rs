fn main() {
    let nombres = (2, 4, 8, 16, 32);

    match nombres {
        (premier, .., dernier) => {
            println!("Voici quelques nombres : {}, {}", premier, dernier);
        }
    }
}
