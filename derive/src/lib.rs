extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field};

#[proc_macro_derive(Context)]
pub fn derive_context(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Deriving kimari::Context only supports a struct."),
    };

    let arms = data
        .fields
        .iter()
        .map(|Field { ident, .. }| {
            quote! {
                stringify!(#ident) => self.#ident.get_from_context(path),
            }
        })
        .collect::<Vec<_>>();

    let name = &input.ident;
    let output = quote! {
        impl kimari::context::Context for #name {
            fn get_from_context<'a, I>(&self, path: I) -> Result<kimari::Value, kimari::context::Error>
            where
                I: IntoIterator<Item = &'a str>,
            {
                let mut path = path.into_iter();
                let name = path.next();
                match name {
                    Some(s) => match s {
                        #(#arms)*
                        _ => return Err(kimari::context::Error::UnexpectedPath([s].into()))
                    },
                    _ => unimplemented!("Value representation of structs is not supported yet."),
                }
            }
        }
    };

    output.into()
}
