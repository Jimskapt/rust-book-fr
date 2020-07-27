use art::types::CouleurPrimaire;
use art::utilitaires::mixer;

fn main() {
    let rouge = CouleurPrimaire::Rouge;
    let jaune = CouleurPrimaire::Jaune;
    mixer(rouge, jaune);
}
