struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

// ANCHOR: here
fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
    Utilisateur {
        actif: true,
        pseudo,
        email,
        nombre_de_connexions: 1,
    }
}
// ANCHOR_END: here

fn main() {
    let utilisateur1 = creer_utilisateur(
        String::from("quelquun@example.com"),
        String::from("pseudoquelconque123"),
    );
}
