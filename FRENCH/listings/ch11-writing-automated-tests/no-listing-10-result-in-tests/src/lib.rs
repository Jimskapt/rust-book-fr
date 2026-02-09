pub fn additione(gauche: u64, droite: u64) -> u64 {
    gauche + droite
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let resultat = additione(2, 2);

        if resultat == 4 {
            Ok(())
        } else {
            Err(String::from("deux plus deux ne vaut pas quatre"))
        }
    }
}
// ANCHOR_END: here
