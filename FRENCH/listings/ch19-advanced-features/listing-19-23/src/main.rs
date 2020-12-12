use std::fmt;

struct Enveloppe(Vec<String>);

impl fmt::Display for Enveloppe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Enveloppe(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
