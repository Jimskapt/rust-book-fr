struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

// ANCHOR: here
fn main() {
    let utilisateur1 = Utilisateur {
        actif: true,
        pseudo: String::from("pseudoquelconque123"),
        email: String::from("quelquun@example.com"),
        nombre_de_connexions: 1,
    };
}
// ANCHOR_END: here
