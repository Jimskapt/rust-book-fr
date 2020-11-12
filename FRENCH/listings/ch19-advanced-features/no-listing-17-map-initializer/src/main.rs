fn main() {
    // ANCHOR: here
    enum Statut {
        Valeur(u32),
        Stop,
    }

    let liste_de_statuts: Vec<Statut> = (0u32..20).map(Statut::Valeur).collect();
    // ANCHOR_END: here
}
