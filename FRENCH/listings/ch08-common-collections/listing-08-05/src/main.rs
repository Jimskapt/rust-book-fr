fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let existe_pas = &v[100];
    let existe_pas = v.get(100);
    // ANCHOR_END: here
}
