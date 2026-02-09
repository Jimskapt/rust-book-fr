fn main() {
// ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let premier = &v[0];

    v.push(6);

    println!("Le premier élément est : {premier}");
    // ANCHOR_END: here
}
