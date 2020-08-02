fn main() {
    // ANCHOR: here
    let mut valeur_du_reglage = Some(5);
    let nouvelle_valeur_du_reglage = Some(10);

    match (valeur_du_reglage, nouvelle_valeur_du_reglage) {
        (Some(_), Some(_)) => {
            println!("Vous ne pouvez pas écraser une valeur déjà existante");
        }
        _ => {
            valeur_du_reglage = nouvelle_valeur_du_reglage;
        }
    }

    println!("Le réglage vaut {:?}", valeur_du_reglage);
    // ANCHOR_END: here
}
