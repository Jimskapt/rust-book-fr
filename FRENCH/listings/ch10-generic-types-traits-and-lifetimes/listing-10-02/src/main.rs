fn main() {
    let liste_de_nombres = vec![34, 50, 25, 100, 65];

    let mut le_plus_grand = &liste_de_nombres[0];

    for nombre in &liste_de_nombres {
        if nombre > le_plus_grand {
            le_plus_grand = nombre;
        }
    }

    println!("Le nombre le plus grand est {le_plus_grand}");

    let liste_de_nombres = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut le_plus_grand = &liste_de_nombres[0];

    for nombre in &liste_de_nombres {
        if nombre > le_plus_grand {
            le_plus_grand = nombre;
        }
    }

    println!("Le nombre le plus grand est {le_plus_grand}");
}
