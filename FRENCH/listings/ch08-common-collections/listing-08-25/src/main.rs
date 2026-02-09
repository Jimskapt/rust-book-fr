fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let texte = "bonjour le monde magnifique monde";

    let mut table = HashMap::new();

    for mot in texte.split_whitespace() {
        let compteur = table.entry(mot).or_insert(0);
        *compteur += 1;
    }

    println!("{table:?}");
    // ANCHOR_END: here
}
