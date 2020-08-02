// ANCHOR: here
use std::ops::Deref;

impl<T> Deref for MaBoite<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
// ANCHOR_END: here

struct MaBoite<T>(T);

impl<T> MaBoite<T> {
    fn new(x: T) -> MaBoite<T> {
        MaBoite(x)
    }
}

fn main() {
    let x = 5;
    let y = MaBoite::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
