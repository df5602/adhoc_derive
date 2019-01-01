extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(FromStr, attributes(adhoc))]
pub fn from_str_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    println!("{:#?}", input);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let result = quote! {
        impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Default::default())
            }
        }
    };
    result.into()
}
