fn main() {
    // ANCHOR: here
    let mut nombre = 5;

    let r1 = &nombre as *const i32;
    let r2 = &mut nombre as *mut i32;
    // ANCHOR_END: here
}
