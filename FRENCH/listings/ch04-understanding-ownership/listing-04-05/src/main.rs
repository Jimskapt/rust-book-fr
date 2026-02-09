fn main() {
    let s1 = String::from("hello");

    let (s2, taille) = calculer_taille(s1);

    println!("La taille de '{s2}' est {taille}.");
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len(); // len() retourne la taille d'une String.

    (s, taille)
}
