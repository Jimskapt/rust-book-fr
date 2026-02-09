fn main() {
    let x = plus_un(5);

    println!("La valeur de x est : {x}");
}

fn plus_un(x: i32) -> i32 {
    x + 1
}
