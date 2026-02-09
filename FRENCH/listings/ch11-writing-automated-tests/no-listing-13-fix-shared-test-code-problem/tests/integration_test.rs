use addition::ajouter_deux;

mod commun;

#[test]
fn cela_ajoute_deux() {
    commun::parametrage();

    let resultat = ajouter_deux(2);
    assert_eq!(resultat, 4);
}
