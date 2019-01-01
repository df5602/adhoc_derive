extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[proc_macro_derive(FromStr, attributes(adhoc))]
pub fn from_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("{:#?}", input);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (regex_string, _regex_span) = extract_regex(&input.attrs);

    let result = quote! {
        impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                lazy_static::lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new(#regex_string).unwrap();
                }

                Ok(Default::default())
            }
        }
    };
    result.into()
}

fn extract_regex(attrs: &[Attribute]) -> (String, Span) {
    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        if meta.name() == "adhoc" {
            match meta {
                Meta::List(meta_list) => {
                    for nested in meta_list.nested.iter() {
                        match nested {
                            NestedMeta::Meta(Meta::NameValue(meta_name_value)) => if meta_name_value.ident == "regex" {
                                match meta_name_value.lit {
                                    Lit::Str(ref lit_str) => return (lit_str.value(), lit_str.span()),
                                    _ => continue,
                                }
                            },
                            _ => continue,
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    panic!("No regex found.");
}