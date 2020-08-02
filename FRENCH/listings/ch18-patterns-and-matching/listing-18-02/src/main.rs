fn main() {
    // ANCHOR: here
    let mut pile = Vec::new();

    pile.push(1);
    pile.push(2);
    pile.push(3);

    while let Some(donnee_du_haut) = pile.pop() {
        println!("{}", donnee_du_haut);
    }
    // ANCHOR_END: here
}
