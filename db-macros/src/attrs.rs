use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, Attribute, Ident, Meta, Token};

use crate::{field::Field, id_type::IdType};

use custom_derive::CustomDerive;

mod custom_derive;

pub struct Attrs {
    pub data: Vec<Meta>,
    pub whole: Vec<Meta>,
    pub both: Vec<Meta>,
    pub derives: HashSet<CustomDerive>,
}

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut data = Vec::new();
        let mut whole = Vec::new();
        let mut both = Vec::new();
        let mut derives = HashSet::new();

        for attr in attrs {
            // println!("Checking attribute");
            let path = attr.path();
            if path.is_ident("data") {
                // println!(" + data");
                let list = attr.meta.require_list()?;
                let meta = list.parse_args()?;
                data.push(meta);
            } else if path.is_ident("whole") {
                // println!(" + whole");

                let list = attr.meta.require_list()?;
                let meta = list.parse_args()?;
                whole.push(meta);
            } else if path.is_ident("db") {
                // println!(" + db");

                let list = attr.meta.require_list()?;
                let punct: Punctuated<CustomDerive, Token![,]> =
                    list.parse_args_with(Punctuated::parse_terminated)?;
                derives.extend(punct.into_iter());
            } else {
                // println!(" + both");
                both.push(attr.meta);
            }
        }

        Ok(Self {
            data,
            whole,
            both,
            derives,
        })
    }
}

impl Attrs {
    pub fn data(&self) -> proc_macro2::TokenStream {
        let attrs = self
            .data
            .iter()
            .chain(self.both.iter())
            .map(|m| quote! {#[#m]});
        quote! {#(#attrs)*}
    }

    pub fn whole(&self) -> proc_macro2::TokenStream {
        let attrs = self
            .whole
            .iter()
            .chain(self.both.iter())
            .map(|m| quote! {#[#m]});
        quote! {#(#attrs)*}
    }

    pub fn derives<'a, I>(
        &self,
        name: &Ident,
        id_name: &Ident,
        id_type: &IdType,
        data_fields: I,
    ) -> TokenStream
    where
        I: Iterator<Item = &'a Field> + Clone,
    {
        let derives = self
            .derives
            .iter()
            .map(|x| x.derive(name, id_name, id_type, data_fields.clone()));
        quote! {#(#derives)*}
    }
}

impl std::fmt::Debug for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn mapping<'a>(x: impl Iterator<Item = &'a Meta>) -> Vec<String> {
            x.map(ToTokens::to_token_stream)
                .map(|x| x.to_string())
                .collect()
        }
        writeln!(
            f,
            "Attrs {{ data: {:?}, whole: {:?}, both: {:?} }}",
            mapping(self.data.iter()),
            mapping(self.whole.iter()),
            mapping(self.both.iter())
        )
    }
}
