fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 => println!("un"),
        2 => println!("deux"),
        3 => println!("trois"),
        _ => println!("n'importe quoi"),
    }
    // ANCHOR_END: here
}
