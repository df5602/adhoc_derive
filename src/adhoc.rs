use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

use crate::transform_idents::TransformIdents;

pub fn from_str_derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (regex_string, _regex_span) = extract_regex(&input.attrs);

    // Validate regex
    match regex::Regex::new(&regex_string) {
        Ok(_) => {}
        Err(e) => panic!("Invalid regex: {:?}", e),
    }

    let (field_ident, parse_expr) = parse_fields(&input.data);

    let result = quote! {
        impl #impl_generics std::str::FromStr for #name #ty_generics #where_clause {
            type Err = Box<std::error::Error>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                struct RegexExtractor<'a> {
                    captures: regex::Captures<'a>,
                }

                impl<'a> RegexExtractor<'a> {
                    fn new(captures: regex::Captures<'a>) -> Self {
                        Self { captures }
                    }

                    fn extract(&self, name: &str) -> std::result::Result<&str, String> {
                        Ok(self
                            .captures
                            .name(name)
                            .ok_or_else(|| format!("no capture group named {}", name))?
                            .as_str())
                    }
                }

                lazy_static::lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new(#regex_string).unwrap();
                }

                let captures = match RE.captures(s) {
                    Some(captures) => captures,
                    None => {
                        return Err("input does not match expected format".into());
                    }
                };

                let extractor = RegexExtractor::new(captures);

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
                            NestedMeta::Meta(Meta::NameValue(meta_name_value)) => {
                                if meta_name_value.ident == "regex" {
                                    match meta_name_value.lit {
                                        Lit::Str(ref lit_str) => {
                                            return (lit_str.value(), lit_str.span());
                                        }
                                        _ => continue,
                                    }
                                }
                            }
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
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    let attributes = parse_attributes(&field.attrs);
                    let field_name = &field.ident.as_ref().unwrap().to_string();

                    idents.push(field.ident.as_ref().unwrap().clone());

                    if let Some(mut expr) = attributes.construct_with {
                        let mut transform_idents = TransformIdents::new();
                        transform_idents.visit_expr_mut(&mut expr);

                        let ts = expr.into_token_stream();
                        parse_exprs.push(quote_spanned! {
                            field.span() => #ts
                        });
                    } else {
                        parse_exprs.push(quote_spanned! {
                            field.span() => extractor.extract(#field_name)?.parse()?
                        });
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    (idents, parse_exprs)
}

#[derive(Debug)]
struct FieldAttributes {
    construct_with: Option<Expr>,
}

fn parse_attributes(attrs: &[Attribute]) -> FieldAttributes {
    let mut attributes = FieldAttributes {
        construct_with: None,
    };

    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        if meta.name() == "adhoc" {
            match meta {
                Meta::List(meta_list) => {
                    for nested in meta_list.nested.iter() {
                        match nested {
                            NestedMeta::Meta(Meta::NameValue(meta_name_value)) => {
                                // Parse #[adhoc(construct_with = "...")]
                                if meta_name_value.ident == "construct_with" {
                                    match meta_name_value.lit {
                                        Lit::Str(ref lit_str) => {
                                            let expr: Expr = parse_str(&lit_str.value()).unwrap();
                                            attributes.construct_with = Some(expr);
                                        }
                                        _ => panic!(
                                            "construct_with must be a function call expression!"
                                        ),
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    attributes
}
