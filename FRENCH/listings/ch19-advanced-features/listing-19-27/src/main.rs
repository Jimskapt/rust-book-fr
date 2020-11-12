fn ajouter_un(x: i32) -> i32 {
    x + 1
}

fn le_faire_deux_fois(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let reponse = le_faire_deux_fois(ajouter_un, 5);

    println!("La réponse est : {}", reponse);
}
