fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Bleu"), 10);
    scores.insert(String::from("Jaune"), 50);

    for (cle, valeur) in &scores {
        println!("{cle} : {valeur}");
    }
    // ANCHOR_END: here
}
