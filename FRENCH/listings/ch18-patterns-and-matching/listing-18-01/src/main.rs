fn main() {
    let couleur_favorite: Option<&str> = None;
    let on_est_mardi = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(couleur) = couleur_favorite {
        println!("Utilisation de votre couleur favorite, {}, comme couleur de fond", couleur);
    } else if on_est_mardi {
        println!("Mardi, c'est le jour du vert !");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Utilisation du violet comme couleur de fond");
        } else {
            println!("Utilisation de l'orange comme couleur de fond");
        }
    } else {
        println!("Utilisation du bleu comme couleur de fond");
    }
}
