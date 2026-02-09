fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // sans problème
    let r2 = &s; // sans problème
    let r3 = &mut s; // GROS PROBLEME
    
    println!("{r1}, {r2}, and {r3}");
    // ANCHOR_END: here
}
