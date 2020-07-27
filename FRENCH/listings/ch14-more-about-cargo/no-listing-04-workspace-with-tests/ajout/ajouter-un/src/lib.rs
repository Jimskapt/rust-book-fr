pub fn ajouter_un(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cela_fonctionne() {
        assert_eq!(3, ajouter_un(2));
    }
}
