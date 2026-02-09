fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let troisieme: &i32 = &v[2];
    println!("Le troisième élément est {}", troisieme);

    let troisieme: Option<&i32> = v.get(2);
    match troisieme {
        Some(troisieme) => println!("Le troisième élément est {}", troisieme),
        None => println!("Il n'y a pas de troisième élément."),
    }
      // ANCHOR_END: here
}
