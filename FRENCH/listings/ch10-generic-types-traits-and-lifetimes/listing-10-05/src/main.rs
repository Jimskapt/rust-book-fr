fn le_plus_grand<T>(liste: &[T]) -> &T {
    let mut le_plus_grand = &liste[0];

    for element in liste {
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

    let liste_de_caracteres = vec!['y', 'm', 'a', 'q'];

    let resultat = le_plus_grand(&liste_de_caracteres);
    println!("Le plus grand caractère est {resultat}");
}
