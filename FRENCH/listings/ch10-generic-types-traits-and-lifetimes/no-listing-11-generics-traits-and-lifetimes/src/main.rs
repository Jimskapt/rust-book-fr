fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultat = la_plus_longue_avec_annonce(
        string1.as_str(),
        string2,
        "Aujourd'hui, c'est l'anniversaire de quelqu'un !",
    );
    println!("La chaîne la plus longue est {resultat}");
}

// ANCHOR: here
use std::fmt::Display;

fn la_plus_longue_avec_annonce<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Annonce ! {ann}");
    if x.len() > y.len() { x } else { y }
}
// ANCHOR_END: here
