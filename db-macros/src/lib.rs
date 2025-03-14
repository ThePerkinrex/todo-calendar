use field::Field;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{format_ident, quote, quote_spanned};
use record::Record;
use syn::{parse_macro_input, Ident};

mod attrs;

mod id_type;

mod field;

mod record;

mod sql;

fn record_with_data_internal(input: Record) -> proc_macro2::TokenStream {
    let Record {
        attrs,
        vis,
        struct_t,
        name,
        _brace: _,
        id_id,
        id_colon,
        id_ty,
        fields,
    } = input;

    let data_name = format_ident!("{}Data", name);
    let id_name = format_ident!("{}Id", name);
    let data_fields = fields.iter().flat_map(|(_, p)| {
        p.pairs().map(|p| {
            let Field {
                attrs,
                vis,
                name,
                colon,
                ty,
            } = p.value();
            let comma = p.punct().into_iter();
            let meta = attrs.data();
            quote! {
                #meta
                #vis #name #colon #ty
                #(#comma)*
            }
        })
    });

    let data_meta = attrs.data();
    let data = quote! {
        #data_meta
        #vis #struct_t #data_name {
            #(#data_fields)*
        }
    };

    let whole_fields = fields.iter().flat_map(|(comma, p)| {
        std::iter::once(quote! {#comma}).chain(p.pairs().map(|p| {
            let Field {
                attrs,
                vis,
                name,
                colon,
                ty,
            } = p.value();
            let comma = p.punct().into_iter();
            let meta = attrs.whole();
            quote! {
                #meta
                #vis #name #colon #ty
                #(#comma)*
            }
        }))
    });

    let whole_meta = attrs.whole();
    let id_fields = id_ty.fields_ts(&id_id, &id_colon);
    let whole = quote! {
        #whole_meta
        #vis #struct_t #name {
            #id_fields
            #(#whole_fields)*
        }
    };

    let from_fields = fields.iter().flat_map(|(comma, p)| {
        std::iter::once(quote! {#comma}).chain(p.pairs().map(|p| {
            let Field {
                attrs: _,
                vis: _,
                name,
                colon,
                ty: _,
            } = p.value();
            let comma = p.punct().into_iter();
            quote! {
                #name #colon data.#name
                #(#comma)*
            }
        }))
    });

    let assign_fields = fields.iter().flat_map(|(_, p)| p).map(
        |Field {
             attrs: _,
             vis: _,
             name,
             colon: _,
             ty: _,
         }| {
            quote! {
                self.#name = data.#name;
            }
        },
    );

    let assign_fields_data = fields.iter().flat_map(|(_, p)| p).map(
        |Field {
             attrs: _,
             vis: _,
             name,
             colon: _,
             ty: _,
         }| {
            quote! {
                #name: self.#name.clone()
            }
        },
    );

    

    let data_fields_structs = fields.iter().flat_map(|(_, p)| p).map(
        |Field {
             attrs: _,
             vis: _,
             name: field,
             colon: _,
             ty,
         }| {
            let name_str = Literal::string(&field.to_string());
            let field = Ident::new(&field.to_string().to_upper_camel_case(), field.span());
            quote! {
                pub struct #field;

                impl crate::db::Field<crate::db::SimpleFieldTable<#name>> for #field {
                    type T = #ty;
                    fn name(table: &crate::db::SimpleFieldTable<#name>) -> &'static str {
                        #name_str
                    }
                }
            }
        },
    );

    let data_fields_mod_name = Ident::new(&format!("{}_data", name.to_string().to_snake_case()), name.span());

    let data_fields_mod = quote!{
        #vis mod #data_fields_mod_name {
            use super::*;
            #(#data_fields_structs)*
        }
    };

    let dbtable = quote_spanned! {name.span() => crate::db::DbTable};
    let assignment_id = id_ty.create_fields_ts(&id_id);

    let from_self = id_ty.create_id_from_fields_ts(&id_name, &id_id, quote! {self});

    let derives = attrs.derives(
        &name,
        &id_name,
        &id_ty,
        fields.iter().flat_map(|(_, x)| x.iter()),
    );

    let imp = quote! {
        impl #name {
            fn from_data(#id_id #id_colon #id_name, data: #data_name) -> Self {
                Self {
                    #assignment_id
                    #(#from_fields)*
                }
            }


            #vis fn set(&mut self, data: #data_name) {
                #(#assign_fields)*
            }
        }

        #[automatically_derived]
        impl #dbtable for #name {
            type Id = #id_name;
            type Data = #data_name;

            fn id(&self) -> Self::Id {
                #from_self
            }

            fn data(&self) -> Self::Data {
                Self::Data {
                    #(#assign_fields_data),*
                }
            }
        }

        #derives
    };

    let id_struct = id_ty.struct_ts(&id_name, &vis, &name);

    quote! {
        #id_struct

        #data

        #whole

        #imp

        #data_fields_mod
    }
}

#[proc_macro]
pub fn record_with_data(input: TokenStream) -> TokenStream {
    let record = parse_macro_input!(input as Record);
    record_with_data_internal(record).into()
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};
    use syn::{parse_quote, File};

    use crate::{attrs::Attrs, id_type::IdType, record::Record};

    #[test]
    fn test_all() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out)
    }

    #[test]
    fn test_attrs() {
        let prog = syn::parse2::<Attrs>(quote! {
            #[data(derive(Debug, Serialize, Deserialize))]
        });
        match prog {
            Ok(x) => println!("Ok {x:?}"),
            Err(x) => println!("Error {x} {:?}", x.span().source_text()),
        }
    }

    #[test]
    fn test_id_type_simple() {
        let prog = syn::parse2::<IdType>(quote! {
            #[derive(Debug, Serialize, Deserialize)]
            i64
        });
        match prog {
            Ok(x) => {
                println!("Ok");
                println!(
                    "{}",
                    x.struct_ts(
                        &parse_quote!(RecipeId),
                        &parse_quote!(),
                        &parse_quote!(Recipe)
                    )
                )
            }
            Err(x) => println!("Error {x} {:?}", x.span().source_text()),
        }
    }
    #[test]
    fn test_id_type_comp() {
        let prog = syn::parse2::<IdType>(quote! {
            #[derive(Debug, Serialize, Deserialize)]
            {
                id1: i64,
                id2: i64
            }
        });
        match prog {
            Ok(x) => {
                println!("Ok");
                println!(
                    "{}",
                    x.struct_ts(
                        &parse_quote!(RecipeId),
                        &parse_quote!(),
                        &parse_quote!(Recipe)
                    )
                )
            }
            Err(x) => panic!("Error {x} {:?}", x.span().source_text()),
        }
    }

    #[test]
    fn test_id_type_comp_gen() {
        let prog = syn::parse2::<IdType>(quote! {
            #[derive(Debug, Serialize, Deserialize)]
            {
                id1: i64,
                id2: i64
            }
        });
        match prog {
            Ok(x) => {
                let macro_out = x.struct_ts(
                    &parse_quote!(RecipeId),
                    &parse_quote!(),
                    &parse_quote!(Recipe),
                );
                let file_out: File = syn::parse2(macro_out).unwrap();
                let expected_out: File = parse_quote! {
                    # [derive (Debug , Serialize , Deserialize)]
                    struct RecipeId { id1 : i64 , id2 : i64 , }
                    mod recipe {
                        pub struct Id1 (pub super::i64) ;
                        impl crate :: db :: IdPart < super::Recipe > for Id1 { }
                        pub struct Id2 (pub super::i64) ;
                        impl crate :: db :: IdPart < super::Recipe > for Id2 { }
                    }
                };
                assert_eq!(
                    expected_out,
                    file_out,
                    "Expected:\n{}\n\nFound:\n{}\n",
                    expected_out.clone().into_token_stream(),
                    file_out.clone().into_token_stream()
                );
            }
            Err(x) => panic!("Error {x} {:?}", x.span().source_text()),
        }
    }

    #[test]
    fn test_all2() {
        let prog: Record = parse_quote! {
            #[data(derive(Debug, Serialize, Deserialize))]
            #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
            pub struct Conversion {
                id: {
                    from_unit: UnitId,
                    to_unit: UnitId
                },
                name: String,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out)
    }

    #[test]
    fn test_all_encode() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    #[encode]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out)
    }

    #[test]
    fn test_all_db_derive_single() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            #[db(DbReadSingle)]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    #[encode]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out)
    }

    #[test]
    fn test_all_db_derive_read_all_single_without_err_nor_orderby() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            #[db(DbReadAll)]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    #[encode]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out);
        let file_out: File = syn::parse2(macro_out).unwrap();
        let expected_out: File = parse_quote! {# [derive (Debug , Serialize , Deserialize)] # [serde (transparent)] # [derive (sqlx :: Type)] # [sqlx (transparent)] pub struct RecipeId (i64) ; impl From < i64 > for RecipeId { fn from (value : i64) -> Self { Self (value) } } # [derive (Debug , Serialize , Deserialize)] pub struct RecipeData { pub name : String , # [serde (deserialize_with = "deserialize_or_default")] pub description : Option < String > , # [serde (deserialize_with = "deserialize_or_default")] pub servings : Option < i64 > , # [serde (deserialize_with = "deserialize_or_default")] pub prep_time : Option < i64 > , # [serde (deserialize_with = "deserialize_or_default")] pub cook_time : Option < i64 > , } # [derive (FromRow)] # [derive (Debug , Serialize , Deserialize)] pub struct Recipe { id : i64 , pub name : String , pub description : Option < String > , pub servings : Option < i64 > , pub prep_time : Option < i64 > , pub cook_time : Option < i64 > , } impl Recipe { fn from_data (id : RecipeId , data : RecipeData) -> Self { Self { id : id . 0 , name : data . name , description : data . description , servings : data . servings , prep_time : data . prep_time , cook_time : data . cook_time , } } pub fn set (& mut self , data : RecipeData) { self . name = data . name ; self . description = data . description ; self . servings = data . servings ; self . prep_time = data . prep_time ; self . cook_time = data . cook_time ; } } # [automatically_derived] impl crate :: db :: DbTable for Recipe { type Id = RecipeId ; type Data = RecipeData ; fn id (& self) -> Self :: Id { RecipeId (self . id) } } # [automatically_derived] impl crate :: db :: DbReadAll for Recipe { async fn get_all (db : & crate :: db :: Db) -> sqlx :: Result < Vec < Self >> { sqlx :: query_as ! (Self , "SELECT * FROM recipe ") . fetch_all (& db . pool) . await } }};
        assert_eq!(file_out, expected_out);
    }

    #[test]
    fn test_all_db_derive_read_all_single_without_err_and_orderby() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            #[db(DbReadAll(order_by(name)))]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    #[encode]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out);
        let file_out: File = syn::parse2(macro_out).unwrap();
        let expected_out: File = parse_quote! {# [derive (Debug , Serialize , Deserialize)] # [serde (transparent)] # [derive (sqlx :: Type)] # [sqlx (transparent)] pub struct RecipeId (i64) ; impl From < i64 > for RecipeId { fn from (value : i64) -> Self { Self (value) } } # [derive (Debug , Serialize , Deserialize)] pub struct RecipeData { pub name : String , # [serde (deserialize_with = "deserialize_or_default")] pub description : Option < String > , # [serde (deserialize_with = "deserialize_or_default")] pub servings : Option < i64 > , # [serde (deserialize_with = "deserialize_or_default")] pub prep_time : Option < i64 > , # [serde (deserialize_with = "deserialize_or_default")] pub cook_time : Option < i64 > , } # [derive (FromRow)] # [derive (Debug , Serialize , Deserialize)] pub struct Recipe { id : i64 , pub name : String , pub description : Option < String > , pub servings : Option < i64 > , pub prep_time : Option < i64 > , pub cook_time : Option < i64 > , } impl Recipe { fn from_data (id : RecipeId , data : RecipeData) -> Self { Self { id : id . 0 , name : data . name , description : data . description , servings : data . servings , prep_time : data . prep_time , cook_time : data . cook_time , } } pub fn set (& mut self , data : RecipeData) { self . name = data . name ; self . description = data . description ; self . servings = data . servings ; self . prep_time = data . prep_time ; self . cook_time = data . cook_time ; } } # [automatically_derived] impl crate :: db :: DbTable for Recipe { type Id = RecipeId ; type Data = RecipeData ; fn id (& self) -> Self :: Id { RecipeId (self . id) } } # [automatically_derived] impl crate :: db :: DbReadAll for Recipe { async fn get_all (db : & crate :: db :: Db) -> sqlx :: Result < Vec < Self >> { sqlx :: query_as ! (Self , "SELECT * FROM recipe ORDER BY name") . fetch_all (& db . pool) . await } }};
        assert_eq!(file_out, expected_out);
    }

    #[test]
    fn test_all_db_derive_read_all_single_with_prop_err() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            #[db(DbReadAll(order_by(what)))]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    #[encode]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out);
        let file_out: File = syn::parse2(macro_out).unwrap();
        let expected_out: File = parse_quote! {# [derive (Debug , Serialize , Deserialize)] # [serde (transparent)] # [derive (sqlx :: Type)] # [sqlx (transparent)] pub struct RecipeId (i64) ; impl From < i64 > for RecipeId { fn from (value : i64) -> Self { Self (value) } } # [derive (Debug , Serialize , Deserialize)] pub struct RecipeData { pub name : String , # [serde (deserialize_with = "deserialize_or_default")] pub description : Option < String > , # [serde (deserialize_with = "deserialize_or_default")] pub servings : Option < i64 > , # [serde (deserialize_with = "deserialize_or_default")] pub prep_time : Option < i64 > , # [serde (deserialize_with = "deserialize_or_default")] pub cook_time : Option < i64 > , } # [derive (FromRow)] # [derive (Debug , Serialize , Deserialize)] pub struct Recipe { id : i64 , pub name : String , pub description : Option < String > , pub servings : Option < i64 > , pub prep_time : Option < i64 > , pub cook_time : Option < i64 > , } impl Recipe { fn from_data (id : RecipeId , data : RecipeData) -> Self { Self { id : id . 0 , name : data . name , description : data . description , servings : data . servings , prep_time : data . prep_time , cook_time : data . cook_time , } } pub fn set (& mut self , data : RecipeData) { self . name = data . name ; self . description = data . description ; self . servings = data . servings ; self . prep_time = data . prep_time ; self . cook_time = data . cook_time ; } } # [automatically_derived] impl crate :: db :: DbTable for Recipe { type Id = RecipeId ; type Data = RecipeData ; fn id (& self) -> Self :: Id { RecipeId (self . id) } fn data (& self) -> Self :: Data { Self :: Data { name : self . name . clone () , description : self . description . clone () , servings : self . servings . clone () , prep_time : self . prep_time . clone () , cook_time : self . cook_time . clone () } } } :: core :: compile_error ! { "what is not a valid property" }};
        assert_eq!(file_out, expected_out);
    }

    #[test]
    #[should_panic]
    fn test_all_db_derive_read_all_single_with_parse_err() {
        let prog: Record = parse_quote! {
            #[derive(Debug, Serialize, Deserialize)]
            #[whole(derive(FromRow))]
            #[db(DbReadAll(order_by()))]
            pub struct Recipe {
                id:
                    #[derive(Debug, Serialize, Deserialize)]
                    #[serde(transparent)]
                    #[encode]
                    i64,
                pub name: String,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub description: Option<String>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub servings: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub prep_time: Option<i64>,
                #[data(serde(deserialize_with = "deserialize_or_default"))]
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out)
    }

    #[test]
    fn test_all_db_derive_update() {
        let prog: Record = parse_quote! {
            #[db(DbUpdate)]
            pub struct Recipe {
                id:
                    i64,
                pub name: String,
                pub description: Option<String>,
                pub servings: Option<i64>,
                pub prep_time: Option<i64>,
                pub cook_time: Option<i64>,
            }
        };
        let macro_out = super::record_with_data_internal(prog);
        println!("{}", macro_out);
        let file_out: File = syn::parse2(macro_out).unwrap();
        let expected_out: File = parse_quote! {
            pub struct RecipeId(i64);
            impl From<i64> for RecipeId {
                fn from(value: i64) -> Self {
                    Self(value)
                }
            }
            pub struct RecipeData {
                pub name: String,
                pub description: Option<String>,
                pub servings: Option<i64>,
                pub prep_time: Option<i64>,
                pub cook_time: Option<i64>,
            }
            pub struct Recipe {
                id: i64,
                pub name: String,
                pub description: Option<String>,
                pub servings: Option<i64>,
                pub prep_time: Option<i64>,
                pub cook_time: Option<i64>,
            }
            impl Recipe {
                fn from_data(id: RecipeId, data: RecipeData) -> Self {
                    Self {
                        id: id.0,
                        name: data.name,
                        description: data.description,
                        servings: data.servings,
                        prep_time: data.prep_time,
                        cook_time: data.cook_time,
                    }
                }
                pub fn set(&mut self, data: RecipeData) {
                    self.name = data.name;
                    self.description = data.description;
                    self.servings = data.servings;
                    self.prep_time = data.prep_time;
                    self.cook_time = data.cook_time;
                }
            }
            #[automatically_derived]
            impl crate::db::DbTable for Recipe {
                type Id = RecipeId;
                type Data = RecipeData;
                fn id(&self) -> Self::Id {
                    RecipeId(self.id)
                }
            }
            #[automatically_derived]
            impl crate::db::DbUpdate for Recipe {
                async fn save(&self, db: &crate::db::Db) -> sqlx::Result<()> {
                    sqlx :: query ! ("UPDATE recipe SET name = $2, description = $3, servings = $4, prep_time = $5, cook_time = $6 WHERE id = $1" , self . id , self . name , self . description , self . servings , self . prep_time , self . cook_time) . execute (& db . pool) . await ? ;
                    Ok(())
                }
            }
        };
        assert_eq!(file_out, expected_out);
    }
}
