use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serial)]
pub fn derive_serial(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Serial for #ident {
            fn deser(buffer: &[u8]) -> Self{
                rmp_serde::from_read(buffer).unwrap()
            }
            fn ser(&self) -> ::std::vec::Vec<u8>{
                rmp_serde::to_vec(self).unwrap()
            }
        }
    };
    output.into()
}