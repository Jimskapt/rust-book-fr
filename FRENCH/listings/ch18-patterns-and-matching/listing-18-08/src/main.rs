fn main() {
    let une_option_quelconque: Option<i32> = None;
    // ANCHOR: here
    let Some(x) = une_option_quelconque;
    // ANCHOR_END: here
}
