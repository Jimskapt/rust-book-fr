extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("La valeur absolue de -3 selon le langage C : {}", abs(-3));
    }
}
