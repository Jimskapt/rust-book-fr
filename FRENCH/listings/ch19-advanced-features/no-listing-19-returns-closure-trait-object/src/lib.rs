fn retourne_une_fermeture() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
