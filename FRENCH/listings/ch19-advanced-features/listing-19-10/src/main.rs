static mut COMPTEUR: u32 = 0;

fn ajouter_au_compteur(valeur: u32) {
    unsafe {
        COMPTEUR += valeur;
    }
}

fn main() {
    ajouter_au_compteur(3);

    unsafe {
        println!("COMPTEUR : {}", COMPTEUR);
    }
}
