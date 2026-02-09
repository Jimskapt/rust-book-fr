fn main() {
    // ANCHOR: here
    let une_valeur_u8 = Some(3u8);
    if let Some(max) = une_valeur_u8 {
        println!("Le maximum est réglé sur {max}");
    }
    // ANCHOR_END: here
}
