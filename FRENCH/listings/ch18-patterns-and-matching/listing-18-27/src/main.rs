fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Nous obtenons 50"),
        Some(n) if n == y => println!("Nous avons une correspondance, n = {}", n),
        _ => println!("Cas par défaut, x = {:?}", x),
    }

    println!("Au final : x = {:?}, y = {}", x, y);
}
