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
