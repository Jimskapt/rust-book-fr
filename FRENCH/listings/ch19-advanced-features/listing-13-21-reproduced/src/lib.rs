struct Compteur {
    compteur: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { compteur: 0 }
    }
}

// ANCHOR: ch19
impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // -- partie masquée ici --
        // ANCHOR_END: ch19
        if self.compteur < 5 {
            self.compteur += 1;
            Some(self.compteur)
        } else {
            None
        }
    }
}

fn main() {}
