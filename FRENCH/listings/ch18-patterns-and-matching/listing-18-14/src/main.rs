struct Point {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("Sur l'axe x à la position {}", x),
        Point { x: 0, y } => println!("Sur l'axe y à la position {}", y),
        Point { x, y } => println!("Sur aucun des axes : ({}, {})", x, y),
    }
}
// ANCHOR_END: here
