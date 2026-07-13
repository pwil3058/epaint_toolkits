// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

extern crate proc_macro;

use heck::KebabCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};

fn acronym(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        if char.is_uppercase() {
            output.push(char);
        }
    }
    output
}

fn abbreviate(input: &str, n: usize) -> String {
    let mut output = String::new();
    for (i, char) in input.chars().enumerate() {
        if i < n {
            output.push(char);
        } else {
            break;
        }
    }
    output.push('.');
    output
}

#[proc_macro_derive(Property, attributes(default))]
pub fn property_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let enum_name = parsed_input.ident;
    let name = enum_name.to_string();
    let prompt = enum_name.to_string() + ":";
    let list_header = abbreviate(&enum_name.to_string(), 2);
    let mut abbrev_tokens = vec![];
    let mut full_tokens = vec![];
    let mut full_variant_tokens = String::new();
    let mut abbrev_variant_tokens = String::new();
    let mut from_tokens = vec![];
    let mut from_f64_tokens = vec![];
    let mut to_f64_tokens = vec![];
    let mut from_u64_tokens = vec![];
    let mut to_u64_tokens = vec![];
    let mut value_tokens = vec![];
    let mut first: Option<Ident> = None;
    let mut default: Option<Ident> = None;
    let fmt_str = format!("'{{}}': Malformed '{name}' value string");
    match parsed_input.data {
        Data::Enum(e) => {
            let mut count: u64 = 1;
            for v in e.variants {
                let v_name = v.ident.clone();
                if first.is_none() {
                    first = Some(v.ident.clone());
                }
                for attr in v.attrs.iter() {
                    if attr.path.is_ident("default") {
                        default = Some(v.ident.clone());
                    }
                }
                let v_abbrev = acronym(&v.ident.to_string());
                let v_full = v.ident.to_string().to_kebab_case();
                let v_full_normal = v.ident.to_string();
                let token = quote! {
                    #v_full,
                };
                value_tokens.push(token);

                let abbrev_token = quote! {
                    #enum_name::#v_name => #v_abbrev,
                };
                abbrev_tokens.push(abbrev_token);
                abbrev_variant_tokens.push_str(format!("{}, ", v_full).as_str());

                let full_token = quote! {
                    #enum_name::#v_name => #v_full,
                };
                full_tokens.push(full_token);
                full_variant_tokens.push_str(format!("{}, ", v_full).as_str());

                let from_token = quote! {
                    #v_abbrev | #v_full | #v_full_normal => Ok(#enum_name::#v_name),
                };
                from_tokens.push(from_token);

                let from_f64_token = quote! {
                    #count => #enum_name::#v_name,
                };
                from_f64_tokens.push(from_f64_token);

                let to_f64_token = quote! {
                    #enum_name::#v_name => #count as f64,
                };
                to_f64_tokens.push(to_f64_token);

                let from_u64_token = quote! {
                    #count => #enum_name::#v_name,
                };
                from_u64_tokens.push(from_u64_token);

                let to_u64_token = quote! {
                    #enum_name::#v_name => #count,
                };
                to_u64_tokens.push(to_u64_token);

                count += 1;
            }
        }
        _ => panic!("'Property' can only be derived for enums."),
    }
    let default_value = if let Some(default) = default {
        default
    } else {
        first.unwrap()
    };
    let tokens = quote! {
        impl PropertyConsts for #enum_name {
            const NAME: &'static str = #name;
            const PROMPT: &'static str = #prompt;
            const LIST_HEADER: &'static str = #list_header;
            const VARIANT_STRS: &'static [&'static str] = &[#full_variant_tokens];
            const ABBREV_VARIANT_STRS: &'static [&'static str] = &[#abbrev_variant_tokens];
        }

        impl PropertyFns for #enum_name {
            fn name(&self) -> &'static str { Self::NAME }

            fn prompt(&self) -> &'static str { Self::PROMPT }

            fn list_header() -> &'static str { Self::LIST_HEADER }

            fn str_values() -> Vec<&'static str> {
                vec![#(#value_tokens)*]
            }

            fn abbrev_value(&self) -> &'static str {
                match *self {
                    #(#abbrev_tokens)*
                }
            }

            fn value(&self) -> &'static str {
                match *self {
                    #(#full_tokens)*
                }
            }
        }

        impl PropertyIfce for #enum_name {}

        impl std::str::FromStr for #enum_name {
            type Err = String;

            fn from_str(string: &str) -> Result<#enum_name, String> {
                match string {
                    #(#from_tokens)*
                    _ => Err(format!(#fmt_str, string)),
                }
            }
        }

        impl std::convert::From<f64> for #enum_name {
            fn from(float: f64) -> #enum_name {
                match float.round() as u64 {
                    #(#from_f64_tokens)*
                    _ => panic!("u64: {} out of range for '{}'", float, #name),
                }
            }
        }

        impl std::convert::From<#enum_name> for f64 {
            fn from(arg: #enum_name) -> f64 {
                match arg {
                    #(#to_f64_tokens)*
                }
            }
        }

        impl std::convert::From<u64> for #enum_name {
            fn from(float: u64) -> #enum_name {
                match float {
                    #(#from_u64_tokens)*
                    _ => panic!("u64: {} out of range for '{}'", float, #name),
                }
            }
        }

        impl std::convert::From<#enum_name> for u64 {
            fn from(arg: #enum_name) -> u64 {
                match arg {
                    #(#to_u64_tokens)*
                }
            }
        }

        impl std::convert::Into<Property> for #enum_name {
            fn into(self) -> Property {
                let value: u64 = self.into();
                Property{
                    property_type: PropertyType::#enum_name,
                    value: value,
                }
            }
        }

        impl std::convert::TryFrom<&Property> for #enum_name {
            type Error = &'static str;

            fn try_from(property: &Property) -> Result<Self, Self::Error> {
                match PropertyType::#enum_name == property.property_type {
                    true => Ok(Self::from(property.value)),
                    false => Err("Incompatible property type")
                }
            }
        }

        impl std::default::Default for #enum_name {
            fn default() -> Self { #enum_name::#default_value }
        }
    };

    proc_macro::TokenStream::from(tokens)
}
