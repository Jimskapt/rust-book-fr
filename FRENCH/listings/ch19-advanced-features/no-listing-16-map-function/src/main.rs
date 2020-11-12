fn main() {
    // ANCHOR: here
    let liste_de_nombres = vec![1, 2, 3];
    let liste_de_chaines: Vec<String> =
        liste_de_nombres.iter().map(ToString::to_string).collect();
    // ANCHOR_END: here
}
