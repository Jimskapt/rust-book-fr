pub fn accueil(nom: &str) -> String {
    String::from("Salut !")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn accueil_contient_le_nom() {
        let resultat = accueil("Carole");
        assert!(
            resultat.contains("Carole"),
            "Le message d'accueil ne contient pas le nom, il vaut `{resultat}`"
        );
    }
    // ANCHOR_END: here
}
