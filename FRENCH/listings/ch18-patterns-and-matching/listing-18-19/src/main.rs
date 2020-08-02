fn main() {
    // ANCHOR: here
    let nombres = (2, 4, 8, 16, 32);

    match nombres {
        (premier, _, troisieme, _, cinquieme) => {
            println!("Voici quelques nombres : {}, {}, {}", premier, troisieme, cinquieme)
        }
    }
    // ANCHOR_END: here
}
