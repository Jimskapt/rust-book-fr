use ajouter_un;
use rand;

fn main() {
    let nombre = 10;
    println!(
        "Hello, world ! {} plus un vaut {} !",
        nombre,
        ajouter_un::ajouter_un(nombre)
    );
}
