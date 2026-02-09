// ANCHOR: here
pub fn ajouter_deux(a: u64) -> u64 {
    a + 3
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cela_ajoute_deux() {
        resultat = ajouter_deux(2);
        assert_eq!(resultat, 4);
    }
}
