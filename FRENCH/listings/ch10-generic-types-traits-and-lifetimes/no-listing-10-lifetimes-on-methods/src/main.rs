struct ExtraitImportant<'a> {
    partie: &'a str,
}

// ANCHOR: 1st
impl<'a> ExtraitImportant<'a> {
    fn niveau(&self) -> i32 {
        3
    }
}
// ANCHOR_END: 1st

// ANCHOR: 3rd
impl<'a> ExtraitImportant<'a> {
    fn annoncer_et_retourner_partie(&self, annonce: &str) -> &str {
        println!("Votre attention s'il vous plaît : {annonce}");
        self.partie
    }
}
// ANCHOR_END: 3rd

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ExtraitImportant {
        partie: first_sentence,
    };
}
