fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() ajoute un littéral de chaîne dans une String
    
    println!("{s}"); // Cela va afficher `hello, world!`
    // ANCHOR_END: here
}
