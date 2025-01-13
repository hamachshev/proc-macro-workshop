use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span());

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named.iter(),
        _ => panic!("expected fields in struct"),
    };
    let fields = fields.map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {
            #name: Option<#ty>
        }
    });
    let gen = quote! {
     pub struct #bident {
            #(#fields,)*
     }
        impl #name{
            fn builder() -> #bident {
            #bident {
                 executable: None,
                 args: None,
                 env: None,
                 current_dir: None,
         }
            }
        }
    };
    gen.into()
}
