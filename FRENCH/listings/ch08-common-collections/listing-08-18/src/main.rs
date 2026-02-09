fn main() {
    // ANCHOR: here
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // notez que s1 a été déplacé ici
                       // et ne pourra plus être utilisé
    // ANCHOR_END: here
}
