fn main() {
    let x = 4;

    let egal_a_x = |z| z == x;

    let y = 4;

    assert!(egal_a_x(y));
}
