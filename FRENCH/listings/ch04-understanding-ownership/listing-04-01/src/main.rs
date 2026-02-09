fn main() {
    // ANCHOR: here
    {                    // s n'est pas en vigueur ici, car elle n'est pas encore déclarée
        let s = "hello"; // s est en vigueur à partir de ce point

        // on fait des choses avec s ici
    }                    // cette portée est maintenant terminée, et s n'est plus en vigueur
    // ANCHOR_END: here
}
