fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 | 2 => println!("un ou deux"),
        3 => println!("trois"),
        _ => println!("quelque chose d'autre"),
    }
    // ANCHOR_END: here
}
