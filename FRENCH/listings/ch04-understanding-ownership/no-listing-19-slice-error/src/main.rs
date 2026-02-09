fn premier_mot(s: &String) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let mot = premier_mot(&s);

    s.clear(); // Erreur !

    println!("Le premier mot est : {mot}");
}
// ANCHOR_END: here
