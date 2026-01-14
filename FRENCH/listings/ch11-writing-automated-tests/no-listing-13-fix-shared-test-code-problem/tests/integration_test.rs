use addition;

mod commun;

#[test]
fn cela_ajoute_deux() {
    commun::parametrage();
    assert_eq!(4, addition::ajouter_deux(2));
}
