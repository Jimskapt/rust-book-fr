pub fn ajouter_deux(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cela_ajoute_deux() {
        let resultat = ajouter_deux(2);
        assert_eq!(resultat, 4);
    }
}
