struct MaBoite<T>(T);

impl<T> MaBoite<T> {
    fn new(x: T) -> MaBoite<T> {
        MaBoite(x)
    }
}

// ANCHOR: here
fn main() {
    let x = 5;
    let y = MaBoite::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// ANCHOR_END: here
