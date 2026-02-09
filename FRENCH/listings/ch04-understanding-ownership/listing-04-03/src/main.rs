fn main() {
  let s = String::from("hello");  // s rentre dans la portée.

  prendre_possession(s);  // La valeur de s est déplacée dans la fonction…
                          // … et n'est plus en vigueur à partir d'ici

  let x = 5;              // x rentre dans la portée.

  creer_copie(x);         // Comme i32 implémente le trait Copy,
                          // x n'est PAS déplacée dans la fonction,
                          // donc on peutcontinuer d'utiliser x ensuite.

} // Ici, x sort de la portée, puis ensuite s. Toutefois, puisque la valeur de
  // s a été déplacée, il ne se passe rien de spécial.

fn prendre_possession(texte: String) { // texte rentre dans la portée.
  println!("{texte}");
} // Ici, texte sort de la portée et `drop` est appelé. La mémoire est libérée.

fn creer_copie(entier: i32) { // entier rentre dans la portée.
  println!("{entier}");
} // Ici, entier sort de la portée. Il ne se passe rien de spécial.
