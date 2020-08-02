struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: Option<u32>,
}

// ANCHOR: here
impl<T> Cache<T>
where
    T: Fn(u32) -> u32
{
    fn new(calcul: T) -> Cache<T> {
        Cache {
            calcul,
            valeur: None,
        }
    }

    fn valeur(&mut self, arg: u32) -> u32 {
        match self.valeur {
            Some(v) => v,
            None => {
                let v = (self.calcul)(arg);
                self.valeur = Some(v);
                v
            },
        }
    }
}
// ANCHOR_END: here

fn main() {}
