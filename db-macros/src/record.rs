use syn::{braced, parse::Parse, punctuated::Punctuated, token, Ident, Token, Visibility};

use crate::{attrs::Attrs, field::Field, id_type::IdType};

pub struct Record {
    pub attrs: Attrs,
    pub vis: Visibility,
    pub struct_t: Token![struct],
    pub name: Ident,
    pub _brace: token::Brace,
    pub id_id: Ident,
    pub id_colon: Token![:],
    pub id_ty: IdType,
    pub fields: Option<(Token![,], Punctuated<Field, Token![,]>)>,
}

impl Parse for Record {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attrs: input.parse()?,
            vis: input.parse()?,
            struct_t: input.parse()?,
            name: input.parse()?,
            _brace: braced!(content in input),
            id_id: content.parse()?,
            id_colon: content.parse()?,
            id_ty: content.parse()?,
            fields: {
                if content.peek(Token![,]) {
                    Some((content.parse()?, Punctuated::parse_terminated(&content)?))
                } else {
                    None
                }
            },
        })
    }
}
