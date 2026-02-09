pub fn ajouter_deux(a: usize) -> usize {
    addition_interne(a, 2)
}

fn addition_interne(gauche: usize, droite: usize) -> usize {
    gauche + droite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interne() {
        resultat = addition_interne(2, 2);
        assert_eq!(resultat, 4);
    }
}
