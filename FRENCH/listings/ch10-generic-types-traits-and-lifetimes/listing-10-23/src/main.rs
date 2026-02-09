// ANCHOR: here
fn main() {
    let string1 = String::from("une longue chaîne est longue");

    {
        let string2 = String::from("xyz");
        let resultat = la_plus_longue(string1.as_str(), string2.as_str());
    }
    println!("La chaîne la plus longue est {resultat}");
}
// ANCHOR_END: here

fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
