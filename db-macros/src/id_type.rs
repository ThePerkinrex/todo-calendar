use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Literal, Span};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::Parse,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{self, Brace},
    Attribute, Ident, Token, Type, Visibility,
};

pub enum IdType {
    Single {
        attrs: Vec<Attribute>,
        vis: Visibility,
        ty: Type,
        encode: bool,
    },
    Multiple {
        attrs: Vec<Attribute>,
        braces: Brace,
        fields: Punctuated<syn::Field, Token![,]>,
    },
}

impl IdType {
    pub fn span(&self) -> Span {
        match self {
            Self::Single { ty, .. } => ty.span(),
            Self::Multiple { braces, .. } => braces.span.join(),
        }
    }
}

impl Parse for IdType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        if input.peek(token::Brace) {
            // println!("After attrs {}", input.cursor().token_stream());
            let content;
            Ok(Self::Multiple {
                attrs,
                braces: braced!(content in input),
                fields: <Punctuated<syn::Field, Token![,]>>::parse_separated_nonempty_with(
                    &content,
                    syn::Field::parse_named,
                )?,
            })
        } else {
            let mut encode = false;
            attrs.retain(|x| {
                if x.path().is_ident("encode") {
                    // println!("Found encode");
                    let mut has_nesting = false;
                    let _ = x.parse_nested_meta(|_| {
                        has_nesting = true;
                        Ok(())
                    });

                    // println!("Found encode: {encode} {has_nesting}");
                    encode = encode || !has_nesting;

                    has_nesting
                } else {
                    true
                }
            });
            Ok(Self::Single {
                attrs,
                vis: input.parse()?,
                ty: input.parse()?,
                encode,
            })
        }
    }
}

impl IdType {
    pub fn struct_ts(
        &self,
        name: &Ident,
        vis: &Visibility,
        table: &Ident,
    ) -> proc_macro2::TokenStream {
        match self {
            Self::Single {
                attrs,
                vis: v,
                encode,
                ty,
            } => {
                let encode = if *encode {
                    quote! {
                        #[derive(sqlx::Type)]
                        #[sqlx(transparent)]
                    }
                } else {
                    quote! {}
                };

                quote! {
                    #(#attrs)*
                    #encode
                    #vis struct #name(#v #ty);

                    impl From<#ty> for #name {
                        fn from(value: #ty) -> Self {
                            Self(value)
                        }
                    }

                    impl crate::db::Field<crate::db::SimpleFieldTable<#table>> for #name {
                        type T = Self;
                        fn name(table: &crate::db::SimpleFieldTable<#table>) -> &'static str {
                            "id"
                        }
                    }

                }
            }
            Self::Multiple {
                attrs,
                braces: _braces,
                fields,
            } => {
                let impls = fields.iter().map(|x| {
                    let ty = &x.ty;
                    let name = x.ident.as_ref().unwrap();
                    let name_str = Literal::string(&name.to_string());
                    let name = Ident::new(&name.to_string().to_upper_camel_case(), name.span());
                    quote! {
                        pub struct #name(pub super::#ty);
                        impl crate::db::IdPart<super::#table> for #name {}
                        impl crate::db::Field<crate::db::SimpleFieldTable<super::#table>> for #name {
                            type T = Self;
                            fn name(table: &crate::db::SimpleFieldTable<super::#table>) -> &'static str {
                                #name_str
                            }
                        }
                    }
                });
                let fields = fields.iter().map(ToTokens::to_token_stream);
                let mod_name = Ident::new(&format!("{}_id", table.to_string().to_snake_case()), table.span());
                quote! {
                    #(#attrs)*
                    #vis struct #name {
                        #(#fields,)*
                    }

                    #vis mod #mod_name {
                        #(#impls)*
                    }
                }
            }
        }
    }

    pub fn fields_ts(&self, id_id: &Ident, id_colon: &Token![:]) -> proc_macro2::TokenStream {
        match self {
            Self::Single {
                attrs: _,
                vis: _,
                encode: _,
                ty,
            } => quote! {
                #id_id #id_colon #ty
            },
            Self::Multiple {
                attrs: _,
                braces: _braces,
                fields,
            } => {
                let fields = fields.iter().map(ToTokens::to_token_stream);
                quote! {
                    #(#fields),*
                }
            }
        }
    }

    pub fn create_fields_ts(&self, id_id: &Ident) -> proc_macro2::TokenStream {
        match self {
            Self::Single {
                attrs: _,
                vis: _,
                encode: _,
                ty: _,
            } => quote! {
                #id_id: #id_id.0
            },
            Self::Multiple {
                attrs: _,
                braces: _braces,
                fields,
            } => {
                let fields = fields.pairs().map(|pair| {
                    let name = pair.value().ident.as_ref().unwrap();
                    let colon = pair.value().colon_token.as_ref().unwrap();

                    quote! {
                        #name #colon #id_id.#name
                    }
                });
                quote! {
                    #(#fields),*
                }
            }
        }
    }

    pub fn create_id_from_fields_ts(
        &self,
        id_name: &Ident,
        id_id: &Ident,
        from: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match self {
            Self::Single {
                attrs: _,
                vis: _,
                encode: _,
                ty: _,
            } => quote! {
                #id_name(#from.#id_id)
            },
            Self::Multiple {
                attrs: _,
                braces: _braces,
                fields,
            } => {
                let fields = fields.pairs().map(|pair| {
                    let name = pair.value().ident.as_ref().unwrap();
                    let colon = pair.value().colon_token.as_ref().unwrap();

                    quote! {
                        #name #colon #from.#name.clone(),
                    }
                });
                quote! {
                    #id_name {
                        #(#fields)*
                    }
                }
            }
        }
    }
}
