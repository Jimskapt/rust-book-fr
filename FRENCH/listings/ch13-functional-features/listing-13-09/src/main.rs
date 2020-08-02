// ANCHOR: here
struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: Option<u32>,
}
// ANCHOR_END: here

fn main() {}
