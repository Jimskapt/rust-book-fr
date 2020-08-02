fn main() {
    // ANCHOR: here
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("On a 50"),
        Some(y) => println!("Correspondance, y = {:?}", y),
        _ => println!("Cas par défaut, x = {:?}", x),
    }

    println!("A la fin : x = {:?}, y = {:?}", x, y);
    // ANCHOR_END: here
}
