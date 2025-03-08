use syn::{parse::Parse, Ident, Token, Type, Visibility};

use crate::attrs::Attrs;

pub struct Field {
    pub attrs: Attrs,
    pub vis: Visibility,
    pub name: Ident,
    pub colon: Token![:],
    pub ty: Type,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.parse()?,
            vis: input.parse()?,
            name: input.parse()?,
            colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}
