fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Bleu"), 10);

    scores.entry(String::from("Jaune")).or_insert(50);
    scores.entry(String::from("Bleu")).or_insert(50);

    println!("{scores:?}");
    // ANCHOR_END: here
}
