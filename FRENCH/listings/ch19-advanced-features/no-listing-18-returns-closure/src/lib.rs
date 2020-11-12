fn retourne_une_fermeture() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
