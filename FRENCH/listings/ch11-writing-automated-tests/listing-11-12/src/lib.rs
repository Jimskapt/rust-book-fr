pub fn ajouter_deux(a: u64) -> u64 {
    addition_interne(a, 2)
}

fn addition_interne(gauche: u64, droite: u64) -> u64 {
    gauche + droite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interne() {
        let resultat = addition_interne(2, 2);
        assert_eq!(resultat, 4);
    }
}
