// ANCHOR: here
fn premier_mot(s: &str) -> &str {
    // ANCHOR_END: here
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let ma_string = String::from("hello world");

    // `premier_mot` fonctionne avec les slices de `String`, que ce soit sur
    // une partie ou sur sur son intégralité.
    let mot = premier_mot(&ma_string[0..6]);
    let mot = premier_mot(&ma_string[..]);

    // `premier_mot` fonctionne également sur des références vers des `String`,
    // qui sont équivalentes à des slices de toute la `String`.
    let mot = premier_mot(&ma_string);

    let mon_litteral_de_chaine = "hello world";

    // `premier_mot` fonctionne avec les slices de littéraux de chaîne, qu'elles
    // soient partielles ou intégrales.
    let mot = premier_mot(&mon_litteral_de_chaine[0..6]);
    let mot = premier_mot(&mon_litteral_de_chaine[..]);

    // Comme les littéraux de chaîne *sont* déjà des slices de chaînes,
    // cela fonctionne aussi, sans la syntaxe de slice !
    let mot = premier_mot(mon_litteral_de_chaine);
}
// ANCHOR_END: usage
