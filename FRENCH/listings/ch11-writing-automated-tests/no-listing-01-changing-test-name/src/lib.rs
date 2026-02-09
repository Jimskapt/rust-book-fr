pub fn additione(gauche: u64, droite: u64) -> u64 {
    gauche + droite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let resultat = additione(2, 2);
        assert_eq!(resultat, 4);
    }
}
