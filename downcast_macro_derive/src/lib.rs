extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Downcast)]
pub fn downcast_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_downcast_macro(&ast)
}

fn impl_downcast_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Downcast for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
    gen.into()
}
