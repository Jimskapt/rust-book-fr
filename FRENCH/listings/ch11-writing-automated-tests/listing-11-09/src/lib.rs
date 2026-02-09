pub struct Supposition {
    valeur: i32,
}

// ANCHOR: here
// -- partie masquée ici --
impl Supposition {
    pub fn new(valeur: i32) -> Supposition {
        if valeur < 1 {
            panic!(
                "La supposition doit être plus grande ou égale à 1, et nous avons {valeur}."
            );
        } else if valeur > 100 {
            panic!(
                "La supposition doit être plus petite ou égale à 100, et nous avons {valeur}."
            );
        }

        Supposition { valeur }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "inférieur ou égal à 100")]
    fn plus_grand_que_100() {
        Supposition::new(200);
    }
}
// ANCHOR_END: here
