use agregateur::{self, ArticleDePresse, Resumable};

fn main() {
    // ANCHOR: here
    let article = ArticleDePresse {
        titre: String::from("Les Penguins ont remporté la Coupe Stanley !"),
        lieu: String::from("Pittsburgh, PA, USA"),
        auteur: String::from("Iceburgh"),
        contenu: String::from(
            "Les Penguins de Pittsburgh sont une nouvelle fois la meilleure \
            équipe de hockey de la LNH.",
        ),
    };

    println!("Nouvel article disponible ! {}", article.resumer());
    // ANCHOR_END: here
}
