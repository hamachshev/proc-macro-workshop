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
        }) => named,
        _ => panic!("expected fields in struct"),
    };
    let n_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {
            #name: Option<#ty>
        }
    });
    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
                 fn #name(&mut self, #name: #ty) -> &mut Self {
                      self.#name = Some(#name);
                      self
          }
        }
    });

    let iter: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let gen = quote! {
     pub struct #bident {
            #(#n_fields,)*
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
        impl #bident {
            #(#methods)*

         pub fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {

    Ok (
        #name {
            #(#iter: self.#iter.clone().ok_or("missing field")?,)*
        }
    )

             }

        }

    };
    gen.into()
}
