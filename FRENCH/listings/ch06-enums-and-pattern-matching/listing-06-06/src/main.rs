fn main() {
    // ANCHOR: here
    let une_valeur_u8 = Some(3u8);
    match une_valeur_u8 {
        Some(max) => println!("Le maximum est réglé sur {max}"),
        _ => (),
    }
    // ANCHOR_END: here
}
