fn main() {
    // ANCHOR: here
    unsafe fn dangereux() {}

    unsafe {
        dangereux();
    }
    // ANCHOR_END: here
}
