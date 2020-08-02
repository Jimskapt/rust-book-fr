fn main() {
    let une_option_quelconque: Option<i32> = None;
    // ANCHOR: here
    if let Some(x) = une_option_quelconque {
        println!("{}", x);
    }
    // ANCHOR_END: here
}
