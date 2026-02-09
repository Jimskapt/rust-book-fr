pub fn ajouter_deux(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajouter_deux_a_deux() {
        let resultat = ajouter_deux(2);
        assert_eq!(resultat, 4);
    }

    #[test]
    fn ajouter_deux_a_trois() {
        let resultat = ajouter_deux(3);
        assert_eq!(resultat, 5);
    }

    #[test]
    fn cent() {
        let resultat = ajouter_deux(100);
        assert_eq!(resultat, 102);
    }
}
