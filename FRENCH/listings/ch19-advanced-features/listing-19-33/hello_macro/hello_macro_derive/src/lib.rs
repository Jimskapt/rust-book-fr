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

// ANCHOR: here
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let nom = &ast.ident;
    let generation = quote! {
        impl HelloMacro for #nom {
            fn hello_macro() {
                println!("Hello, Macro ! Mon nom est {}", stringify!(#nom));
            }
        }
    };
    generation.into()
}
// ANCHOR_END: here
