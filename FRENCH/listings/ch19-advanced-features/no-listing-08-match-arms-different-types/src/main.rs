fn main() {
    let supposition = "3";
    // ANCHOR: here
    let supposition = match supposition.trim().parse() {
        Ok(_) => 5,
        Err(_) => "salut",
    };
    // ANCHOR_END: here
}
