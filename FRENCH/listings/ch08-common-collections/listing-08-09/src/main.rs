fn main() {
    // ANCHOR: here
    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let ligne = vec![
        Cellule::Int(3),
        Cellule::Text(String::from("bleu")),
        Cellule::Float(10.12),
    ];
    // ANCHOR_END: here
}
