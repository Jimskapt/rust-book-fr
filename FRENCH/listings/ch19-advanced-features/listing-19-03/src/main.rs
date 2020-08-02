fn main() {
    // ANCHOR: here
    let mut nombre = 5;

    let r1 = &nombre as *const i32;
    let r2 = &mut nombre as *mut i32;

    unsafe {
        println!("r1 vaut : {}", *r1);
        println!("r2 vaut : {}", *r2);
    }
    // ANCHOR_END: here
}
