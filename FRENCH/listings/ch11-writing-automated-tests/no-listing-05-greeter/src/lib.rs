pub fn accueil(nom: &str) -> String {
    format!("Salut, {nom} !")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accueil_contient_le_nom() {
        let resultat = accueil("Carole");
        assert!(resultat.contains("Carole"));
    }
}
