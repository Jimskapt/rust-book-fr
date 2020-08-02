fn main() {
    // ANCHOR: here
    let s = Some(String::from("Salutations !"));

    if let Some(_s) = s {
        println!("j'ai trouvé une chaine de caractères");
    }

    println!("{:?}", s);
    // ANCHOR_END: here
}
