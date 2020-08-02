fn main() {
    // ANCHOR: here
    let v = vec!['a', 'b', 'c'];

    for (indice, valeur) in v.iter().enumerate() {
        println!("{} est à l'indice {}", valeur, indice);
    }
    // ANCHOR_END: here
}
