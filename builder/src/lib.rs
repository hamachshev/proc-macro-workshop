use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataStruct, DeriveInput, Field, Fields,
    FieldsNamed, Ident, Token,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span());

    let fields = match &ast.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named, .. }) => named.iter(),
            _ => panic!("expected fields in struct"),
        },
        _ => panic!("expected fields in struct"),
    };

    let mut optionized = Punctuated::new();

    let fields = fields.map(|f| {});
    let gen = quote! {
     pub struct #bident {
            #(#fields,)*
     }
        impl #name{
            fn builder() -> #bident {
            #bident {
                }
            }
        }
    };
    gen.into()
}
