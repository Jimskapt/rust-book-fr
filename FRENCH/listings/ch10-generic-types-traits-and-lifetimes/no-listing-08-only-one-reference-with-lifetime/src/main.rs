fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let resultat = la_plus_longue(string1.as_str(), string2);
    println!("La chaîne la plus longue est {resultat}");
}

// ANCHOR: here
fn la_plus_longue<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// ANCHOR_END: here
