use std::ops::Add;

struct Milimetres(u32);
struct Metres(u32);

impl Add<Metres> for Milimetres {
    type Output = Milimetres;

    fn add(self, other: Metres) -> Milimetres {
        Milimetres(self.0 + (other.0 * 1000))
    }
}
