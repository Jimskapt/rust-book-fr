fn main() {
    // ANCHOR: here
    use std::slice;

    let addresse = 0x01234usize;
    let r = addresse as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // ANCHOR_END: here
}
