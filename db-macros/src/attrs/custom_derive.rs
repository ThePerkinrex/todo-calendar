use heck::ToSnakeCase;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{parenthesized, parse::Parse, parse_quote, token::Paren, Ident};

use crate::{
    field::Field,
    id_type::IdType,
    sql::{SqlAndJoin, SqlEquals, SqlVar},
};

mod custom_derive_kw {
    syn::custom_keyword!(DbCreate);
    syn::custom_keyword!(DbCreateWithId);
    syn::custom_keyword!(DbReadSingle);
    syn::custom_keyword!(DbReadAll);
    syn::custom_keyword!(DbUpdate);
    syn::custom_keyword!(DbDelete);
    syn::custom_keyword!(order_by);
}

pub enum OrderBy {
    None,
    By {
        paren: Paren,
        _kw: custom_derive_kw::order_by,
        _paren2: Paren,
        prop: Ident,
    },
}

impl Parse for OrderBy {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            let content;
            let paren = parenthesized!(content in input);
            let lookahead = content.lookahead1();
            if lookahead.peek(custom_derive_kw::order_by) {
                let kw = content.parse()?;
                let content2;
                let paren2 = parenthesized!(content2 in content);
                let prop = content2.parse()?;
                Ok(Self::By {
                    paren,
                    _kw: kw,
                    _paren2: paren2,
                    prop,
                })
            } else {
                Err(lookahead.error())
            }
        } else {
            Ok(Self::None)
        }
    }
}

impl OrderBy {
    pub fn check<I>(&self, mut iter: I) -> syn::Result<CheckedOrderBy<'_>>
    where
        I: Iterator<Item = Ident>,
    {
        match self {
            Self::None => Ok(CheckedOrderBy::None),
            Self::By { prop, paren, .. } => {
                if iter.any(|i| &i == prop) {
                    Ok(CheckedOrderBy::By(prop))
                } else {
                    Err(syn::Error::new(
                        paren.span.join(),
                        format!("{prop} is not a valid property"),
                    ))
                }
            }
        }
    }
}

pub enum CheckedOrderBy<'a> {
    None,
    By(&'a Ident),
}

impl<'a> std::fmt::Display for CheckedOrderBy<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::By(ident) => write!(f, "ORDER BY {ident}"),
        }
    }
}

#[allow(clippy::enum_variant_names)]
pub enum CustomDerive {
    DbCreate(custom_derive_kw::DbCreate),
    DbCreateWithId(custom_derive_kw::DbCreateWithId),
    DbReadSingle(custom_derive_kw::DbReadSingle),
    DbReadAll(custom_derive_kw::DbReadAll, OrderBy),
    DbUpdate(custom_derive_kw::DbUpdate),
    DbDelete(custom_derive_kw::DbDelete),
}

impl PartialEq for CustomDerive {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::DbCreate(_), Self::DbCreate(_))
                | (Self::DbCreateWithId(_), Self::DbCreateWithId(_))
                | (Self::DbReadSingle(_), Self::DbReadSingle(_))
                | (Self::DbReadAll(_, _), Self::DbReadAll(_, _))
                | (Self::DbUpdate(_), Self::DbUpdate(_))
                | (Self::DbDelete(_), Self::DbDelete(_))
        )
    }
}

impl Eq for CustomDerive {}

impl std::hash::Hash for CustomDerive {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Parse for CustomDerive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(custom_derive_kw::DbCreate) {
            Ok(Self::DbCreate(input.parse()?))
        } else if lookahead.peek(custom_derive_kw::DbCreateWithId) {
            Ok(Self::DbCreateWithId(input.parse()?))
        } else if lookahead.peek(custom_derive_kw::DbReadSingle) {
            Ok(Self::DbReadSingle(input.parse()?))
        } else if lookahead.peek(custom_derive_kw::DbReadAll) {
            Ok(Self::DbReadAll(input.parse()?, input.parse()?))
        } else if lookahead.peek(custom_derive_kw::DbUpdate) {
            Ok(Self::DbUpdate(input.parse()?))
        } else if lookahead.peek(custom_derive_kw::DbDelete) {
            Ok(Self::DbDelete(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl CustomDerive {
    fn db_create<'a, I>(
        &self,
        name: &Ident,
        id_name: &Ident,
        id_type: &IdType,
        data_fields: I,
        table_name: Ident,
        db_create: &custom_derive_kw::DbCreate,
    ) -> TokenStream
    where
        I: Iterator<Item = &'a Field>,
    {
        if let IdType::Single { .. } = id_type {
            let (args, columns): (Vec<_>, Vec<_>) = data_fields
                .map(|x| {
                    let i = &x.name;
                    (quote! {data.#i}, x.name.to_string())
                })
                .unzip();

            let columns_str = columns.join(", ");
            let vars = (1..=columns.len())
                .map(|x| SqlVar(x).to_string())
                .collect::<Vec<_>>()
                .join(", ");

            let query = Literal::string(&format!(
                "INSERT INTO {}({}) VALUES({}) RETURNING id",
                table_name, columns_str, vars
            ));

            quote! {
                #[automatically_derived]
                impl crate::db::#db_create for #name {
                    async fn new(db: &Db, data: Self::Data) -> sqlx::Result<Self> {
                        let query = sqlx::query!(
                            #query,
                            #(#args),*
                        );
                        let id = #id_name(query.fetch_one(&db.pool).await?.id);

                        Ok(Self::from_data(id, data))
                    }
                }
            }
        } else {
            syn::Error::new(id_type.span(), "`id` can't be complex").into_compile_error()
        }
    }

    fn db_create_with_id<'a, I>(
        &self,
        name: &Ident,
        _id_name: &Ident,
        id_type: &IdType,
        data_fields: I,
        table_name: Ident,
        db_create_with_id: &custom_derive_kw::DbCreateWithId,
    ) -> TokenStream
    where
        I: Iterator<Item = &'a Field>,
    {
        let id_columns: Box<dyn Iterator<Item = String>> = match id_type {
            IdType::Single { .. } => Box::new(std::iter::once("id".to_string())),
            IdType::Multiple { fields, .. } => Box::new(
                fields
                    .iter()
                    .flat_map(|x| x.ident.iter())
                    .map(ToString::to_string),
            ),
        };

        let (args, columns): (Vec<_>, Vec<_>) = id_columns
            .map(|x| {
                let i = Ident::new(&x, Span::call_site());
                (quote! {id.#i}, x)
            })
            .chain(data_fields.map(|x| {
                let i = &x.name;
                (quote! {data.#i}, x.name.to_string())
            }))
            .unzip();

        let columns_str = columns.join(", ");
        let vars = (1..=columns.len())
            .map(|x| SqlVar(x).to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let query = Literal::string(&format!(
            "INSERT INTO {}({}) VALUES({})",
            table_name, columns_str, vars
        ));

        quote! {
            #[automatically_derived]
            impl crate::db::#db_create_with_id for #name {
                async fn new(db: &Db, id: Self::Id, data: Self::Data) -> sqlx::Result<Self> {
                    let query = sqlx::query!(
                        #query,
                        #(#args),*
                    );
                    query.execute(&db.pool).await?;

                    Ok(Self::from_data(id, data))
                }
            }
        }
    }

    pub fn derive<'a, I>(
        &self,
        name: &Ident,
        id_name: &Ident,
        id_type: &IdType,
        data_fields: I,
    ) -> TokenStream
    where
        I: Iterator<Item = &'a Field>,
    {
        let table_name = Ident::new(&name.to_string().to_snake_case(), name.span());
        match self {
            Self::DbCreate(db_create) => {
                self.db_create(name, id_name, id_type, data_fields, table_name, db_create)
            }
            Self::DbCreateWithId(db_create_with_id) => self.db_create_with_id(
                name,
                id_name,
                id_type,
                data_fields,
                table_name,
                db_create_with_id,
            ),
            Self::DbReadSingle(db_read_single) => {
                let where_clause = match id_type {
                    IdType::Single { .. } => "id = $1".to_string(),
                    IdType::Multiple { fields, .. } => SqlAndJoin(|| {
                        fields
                            .iter()
                            .flat_map(|x| x.ident.iter())
                            .enumerate()
                            .map(|(i, x)| SqlEquals(x, SqlVar(i + 1)))
                    })
                    .to_string(),
                };

                let query = Literal::string(&format!(
                    "SELECT * FROM {} WHERE {}",
                    table_name, where_clause
                ));

                let args = match id_type {
                    IdType::Single { .. } => vec![quote! {id}],
                    IdType::Multiple { fields, .. } => fields
                        .iter()
                        .flat_map(|x| x.ident.iter())
                        .map(|x| quote! {id.#x})
                        .collect(),
                };

                quote! {
                    #[automatically_derived]
                    impl crate::db::#db_read_single for #name {
                        async fn get(db: &crate::db::Db, id: &Self::Id) -> sqlx::Result<Option<Self>> {
                            sqlx::query_as!(Self, #query, #(#args),*).fetch_optional(&db.pool).await
                        }
                    }
                }
            }
            Self::DbReadAll(db_read_all, order_by) => {
                let id_fields: Box<dyn Iterator<Item = Ident>> = match id_type {
                    IdType::Single { .. } => Box::new(std::iter::once_with(|| parse_quote!(id))),
                    IdType::Multiple { fields, .. } => {
                        Box::new(fields.iter().flat_map(|x| x.ident.iter()).cloned())
                    }
                };
                let checked_order =
                    match order_by.check(id_fields.chain(data_fields.map(|f| f.name.clone()))) {
                        Ok(x) => x,
                        Err(e) => return e.to_compile_error(),
                    };

                let query = Literal::string(&format!("SELECT * FROM {table_name} {checked_order}"));

                quote! {
                    #[automatically_derived]
                    impl crate::db::#db_read_all for #name {
                        async fn get_all(db: &crate::db::Db) -> sqlx::Result<Vec<Self>> {
                            sqlx::query_as!(Self, #query).fetch_all(&db.pool).await
                        }
                    }
                }
            }
            Self::DbUpdate(db_update) => {
                let (where_clause, vars, max_id) = match id_type {
                    IdType::Single { .. } => ("id = $1".to_string(), quote! {self.id}, 1),
                    IdType::Multiple { fields, .. } => (
                        SqlAndJoin(|| {
                            fields
                                .iter()
                                .flat_map(|x| x.ident.iter())
                                .enumerate()
                                .map(|(i, x)| SqlEquals(x, SqlVar(i + 1)))
                        })
                        .to_string(),
                        {
                            let vars = fields
                                .iter()
                                .flat_map(|x| x.ident.iter())
                                .map(|x| quote! {self.#x});

                            quote! {#(#vars),*}
                        },
                        fields.len(),
                    ),
                };

                let (set_clauses, data_vars): (Vec<_>, Vec<_>) = data_fields
                    .enumerate()
                    .map(|(i, f)| {
                        (SqlEquals(&f.name, SqlVar(i + max_id + 1)).to_string(), {
                            let name = &f.name;
                            quote! {self.#name}
                        })
                    })
                    .unzip();

                let set_clauses = set_clauses.join(", ");

                let query = Literal::string(&format!(
                    "UPDATE {} SET {} WHERE {}",
                    table_name, set_clauses, where_clause
                ));

                quote! {
                    #[automatically_derived]
                    impl crate::db::#db_update for #name {
                        async fn save(&self, db: &crate::db::Db) -> sqlx::Result<()> {
                            sqlx::query!(#query, #vars, #(#data_vars),*).execute(&db.pool).await?;
                            Ok(())
                        }
                    }
                }
            }
            Self::DbDelete(db_delete) => {
                let where_clause = match id_type {
                    IdType::Single { .. } => "id = $1".to_string(),
                    IdType::Multiple { fields, .. } => SqlAndJoin(|| {
                        fields
                            .iter()
                            .flat_map(|x| x.ident.iter())
                            .enumerate()
                            .map(|(i, x)| SqlEquals(x, SqlVar(i + 1)))
                    })
                    .to_string(),
                };

                let query = Literal::string(&format!(
                    "DELETE FROM {} WHERE {}",
                    table_name, where_clause
                ));

                let args = match id_type {
                    IdType::Single { .. } => vec![quote! {id}],
                    IdType::Multiple { fields, .. } => fields
                        .iter()
                        .flat_map(|x| x.ident.iter())
                        .map(|x| quote! {id.#x})
                        .collect(),
                };

                quote! {
                    #[automatically_derived]
                    impl crate::db::#db_delete for #name {
                        async fn delete_static(db: &Db, id: &Self::Id) -> sqlx::Result<()> {
                            let query = sqlx::query!(
                                #query, #(#args),*
                            );
                            query.execute(&db.pool).await?;
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}
