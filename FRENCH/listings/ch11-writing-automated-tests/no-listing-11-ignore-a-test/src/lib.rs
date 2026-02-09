pub fn additione(gauche: u64, droite: u64) -> u64 {
    gauche + droite
}

// ANCHOR: here

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    let resultat = additione(2, 2);
    assert_eq!(resultat, 4);
    }

    #[test]
    #[ignore]
    fn test_long() {
        // du code qui prend une heure à s'exécuter
    }
}
// ANCHOR_END: here
