// ANCHOR: here
struct MessageQuitter; // une structure unité
struct MessageDeplacer {
    x: i32,
    y: i32,
}
struct MessageEcrire(String); // une structure tuple
struct MessageChangerCouleur(i32, i32, i32); // une structure tuple
                                             // ANCHOR_END: here
fn main() {}
