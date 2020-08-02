struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: Option<u32>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn appel_avec_differentes_valeurs() {
        let mut c = Cache::new(|a| a);

        let v1 = c.valeur(1);
        let v2 = c.valeur(2);

        assert_eq!(v2, 2);
    }
    // ANCHOR_END: here
}
