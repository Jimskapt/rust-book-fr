fn main() {
    afficher_mesure_avec_unite(5, 'h');
}

fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est : {valeur}{unite}");
}
