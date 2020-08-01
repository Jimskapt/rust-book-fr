extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construit une représentation du code Rust en arborescence
    // syntaxique que nous pouvons manipuler
    let ast = syn::parse(input).unwrap();

    // Construit l'implémentation du trait
    impl_hello_macro(&ast)
}
