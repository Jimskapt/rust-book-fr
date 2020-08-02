fn main() {
    let x = 4;

    fn egal_a_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(egal_a_x(y));
}
