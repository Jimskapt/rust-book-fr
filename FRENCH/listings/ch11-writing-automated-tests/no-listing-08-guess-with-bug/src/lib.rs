pub struct Supposition {
    valeur: i32,
}

// ANCHOR: here
// -- partie masquée ici --
impl Supposition {
    pub fn new(valeur: i32) -> Supposition {
        if valeur < 1 {
            panic!("La supposition doit se trouver entre 1 et 100, et nous avons {valeur}.");
        }

        Supposition { valeur }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn plus_grand_que_100() {
        Supposition::new(200);
    }
}
