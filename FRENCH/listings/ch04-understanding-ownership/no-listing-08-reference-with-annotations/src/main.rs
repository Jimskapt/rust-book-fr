fn main() {
    let s1 = String::from("hello");

    let long = calculer_taille(&s1);

    println!("La taille de '{s1}' est {long}.");
}

// ANCHOR: here
fn calculer_taille(s: &String) -> usize { // s est une référence à une String
  s.len()
} // Ici, s sort de la portée. Mais comme s n'a pas la possession de ce à quoi 
  // elle fait référence, il ne se passe rien.
// ANCHOR_END: here
