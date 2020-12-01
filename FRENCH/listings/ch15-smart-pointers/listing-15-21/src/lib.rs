pub trait Messager {
    fn envoyer(&self, msg: &str);
}

pub struct TraqueurDeLimite<'a, T: Messager> {
    messager: &'a T,
    valeur: usize,
    max: usize,
}

impl<'a, T> TraqueurDeLimite<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &T, max: usize) -> TraqueurDeLimite<T> {
        TraqueurDeLimite {
            messager,
            valeur: 0,
            max,
        }
    }

    pub fn set_valeur(&mut self, valeur: usize) {
        self.valeur = valeur;

        let pourcentage_du_maximum = self.valeur as f64 / self.max as f64;

        if pourcentage_du_maximum >= 1.0 {
            self.messager.envoyer("Erreur : vous avez dépassé votre quota !");
        } else if pourcentage_du_maximum >= 0.9 {
            self.messager
                .envoyer("Avertissement urgent : vous avez utilisé 90% de votre quota !");
        } else if pourcentage_du_maximum >= 0.75 {
            self.messager
                .envoyer("Avertissement : vous avez utilisé 75% de votre quota !");
        }
    }
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    struct MessagerMock {
        messages_envoyes: Vec<String>,
    }

    impl MessagerMock {
        fn new() -> MessagerMock {
            MessagerMock {
                messages_envoyes: vec![],
            }
        }
    }

    impl Messager for MessagerMock {
        fn envoyer(&self, message: &str) {
            self.messages_envoyes.push(String::from(message));
        }
    }

    #[test]
    fn envoi_d_un_message_d_avertissement_superieur_a_75_pourcent() {
        let messager_mock = MessagerMock::new();
        let mut traqueur = TraqueurDeLimite::new(&messager_mock, 100);

        traqueur.set_valeur(80);

        assert_eq!(messager_mock.messages_envoyes.len(), 1);
    }
}
// ANCHOR_END: here
