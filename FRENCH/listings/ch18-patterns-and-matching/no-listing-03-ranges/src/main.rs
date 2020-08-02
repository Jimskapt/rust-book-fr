fn main() {
    // ANCHOR: here
    let x = 5;

    match x {
        1..=5 => println!("de un à cinq"),
        _ => println!("quelque chose d'autre"),
    }
    // ANCHOR_END: here
}
