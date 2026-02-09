// ANCHOR: here
fn le_plus_grand(liste: &[i32]) -> &i32 {
    let mut le_plus_grand = &liste[0];

    for &element in liste {
        if element > le_plus_grand {
            le_plus_grand = element;
        }
    }

    le_plus_grand
}

fn main() {
    let liste_de_nombres = vec![34, 50, 25, 100, 65];

    let resultat = le_plus_grand(&liste_de_nombres);
    println!("Le nombre le plus grand est {resultat}");
    // ANCHOR_END: here
    assert_eq!(resultat, 100);
    // ANCHOR: here

    let liste_de_nombres = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let resultat = le_plus_grand(&liste_de_nombres);
    println!("Le nombre le plus grand est {resultat}");
    // ANCHOR_END: here
    assert_eq!(*resultat, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
