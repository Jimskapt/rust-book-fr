pub fn affiche_et_retourne_10(a: i32) -> i32 {
    println!("J'ai obtenu la valeur {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ce_test_reussit() {
        let valeur = affiche_et_retourne_10(4);
        assert_eq!(valeur, 10);
    }

    #[test]
    fn ce_test_echoue() {
        let valeur = affiche_et_retourne_10(8);
        assert_eq!(valeur, 5);
    }
}
