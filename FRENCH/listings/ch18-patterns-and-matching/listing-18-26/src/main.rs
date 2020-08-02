fn main() {
    // ANCHOR: here
    let nombre = Some(4);

    match nombre {
        Some(x) if x < 5 => println!("moins que cinq : {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    // ANCHOR_END: here
}
