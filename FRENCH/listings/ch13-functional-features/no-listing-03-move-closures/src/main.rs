fn main() {
    let x = vec![1, 2, 3];

    let egal_a_x = move |z| z == x;

    println!("On ne peut pas utiliser x ici : {:?}", x);

    let y = vec![1, 2, 3];

    assert!(egal_a_x(y));
}
