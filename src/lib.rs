#![recursion_limit="128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::*;
use syn::spanned::Spanned;

#[proc_macro_derive(FromStr, attributes(adhoc))]
pub fn from_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    //println!("{:#?}", input);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (regex_string, _regex_span) = extract_regex(&input.attrs);

    let (field_ident, parse_expr) = parse_fields(&input.data);

    let result = quote! {
        impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
            type Err = Box<std::error::Error>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                lazy_static::lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new(#regex_string).unwrap();
                }

                let captures = match RE.captures(s) {
                    Some(captures) => captures,
                    None => {
                        return Err("input does not match expected format".into());
                    }
                };

                Ok(Self{#(#field_ident: #parse_expr,)*})
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

fn parse_fields(data: &Data) -> (Vec<Ident>, Vec<proc_macro2::TokenStream>) {
    let mut idents = Vec::new();
    let mut parse_exprs = Vec::new();
    match *data {
        Data::Struct(ref data_struct) => {
            match data_struct.fields {
                Fields::Named(ref fields) => {
                    for field in fields.named.iter() {
                        let field_name = &field.ident.as_ref().unwrap().to_string();
                        idents.push(field.ident.as_ref().unwrap().clone());
                        parse_exprs.push(quote_spanned! {
                            field.span() => captures.name(#field_name).unwrap().as_str().parse()?
                        });
                    }
                },
                _ => unimplemented!(),
            }
        },
        _ => unimplemented!(),
    }
    (idents, parse_exprs)
}