fn main() {
    // ANCHOR: here
    let x = 'c';

    match x {
        'a'..='j' => println!("lettre ASCII du début"),
        'k'..='z' => println!("lettre ASCII de la fin"),
        _ => println!("autre chose"),
    }
    // ANCHOR_END: here
}
