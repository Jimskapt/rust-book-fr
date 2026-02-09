fn main() {
    let mut compteur = 0;
    'increment: loop {
        println!("compteur = {compteur}");
        let mut restant = 10;

        loop {
            println!("restant = {restant}");
            if restant == 9 {
                break;
            }
            if compteur == 2 {
                break 'increment;
            }
            restant -= 1;
        }

        compteur += 1;
    }
    println!("Fin du compteur = {compteur}");
}
