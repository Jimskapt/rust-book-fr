// ANCHOR: here
fn premier_mot(s: &str) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = premier_mot(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = premier_mot(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = premier_mot(my_string_literal);
}
