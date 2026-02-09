fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultat = la_plus_longue(string1.as_str(), string2);
    println!("La chaîne la plus longue est {resultatl}");
}

// ANCHOR: here
fn la_plus_longue<'a>(x: &str, y: &str) -> &'a str {
    let resultat = String::from("très longue chaîne");
    resultat.as_str()
}
// ANCHOR_END: here
