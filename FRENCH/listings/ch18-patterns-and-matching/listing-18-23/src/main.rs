fn main() {
    // ANCHOR: here
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origine = Point { x: 0, y: 0, z: 0 };

    match origine {
        Point { x, .. } => println!("x vaut {}", x),
    }
    // ANCHOR_END: here
}
