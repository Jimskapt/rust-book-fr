// ANCHOR: here
struct MaBoite<T>(T);

impl<T> MaBoite<T> {
    fn new(x: T) -> MaBoite<T> {
        MaBoite(x)
    }
}
// ANCHOR_END: here

fn main() {}
