use addition::ajouter_deux;

#[test]
fn cela_ajoute_deux() {
    let resultat = addition::ajouter_deux(2);
    assert_eq!(resultat, 4);
}
