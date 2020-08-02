fn main() {
    // ANCHOR: here
    let fermeture_exemple = |x| x;

    let s = fermeture_exemple(String::from("hello"));
    let n = fermeture_exemple(5);
    // ANCHOR_END: here
}
