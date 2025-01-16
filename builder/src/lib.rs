use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, AngleBracketedGenericArguments, Data, DataStruct,
    DeriveInput, Fields, FieldsNamed, GenericArgument, Ident, Path, PathArguments, PathSegment,
    Token, Type, TypePath,
};

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
        if is_option(&f.ty) {
            quote! {
                 #name: #ty
            }
        } else {
            quote! {
                #name: Option<#ty>
            }
        }
    });
    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        if is_option(&f.ty) {
            let ty = get_type(&f.ty);
            quote! {
                      fn #name(&mut self, #name: #ty) -> &mut Self {
                           self.#name = Some(#name);
                           self
               }
            }
        } else {
            let ty = &f.ty;
            quote! {
                     fn #name(&mut self, #name: #ty) -> &mut Self {
                          self.#name = Some(#name);
                          self
              }
            }
        }
    });

    let non_option_iter: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            if !is_option(&f.ty) {
                Some(&f.ident)
            } else {
                None
            }
        })
        .collect();
    let option_iter: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            if is_option(&f.ty) {
                Some(&f.ident)
            } else {
                None
            }
        })
        .collect();

    let default_field_values = fields.iter().map(|f| {
        let field_ident = &f.ident;
        quote! {
                #field_ident: None
        }
    });

    fn is_option(ty: &Type) -> bool {
        if let Type::Path(
            TypePath {
                path: Path { segments, .. },
                ..
            },
            ..,
        ) = ty
        {
            if let Some(PathSegment { ident, .. }) = segments.first() {
                if ident.to_string() == "Option" {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn get_type(ty: &Type) -> &Punctuated<GenericArgument, Token![,]> {
        match ty {
            Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) => match &segments[0] {
                PathSegment {
                    arguments:
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
                    ..
                } => args,
                _ => panic!("help"),
            },
            _ => panic!("help"),
        }
    }
    //Type::Path(
    //         TypePath {
    //             qself: None,
    //             path: Path {
    //                 segments: [
    //                     PathSegment {
    //                         ident: "Option",

    let gen = quote! {
     pub struct #bident {
            #(#n_fields,)*
     }
        impl #name{
            fn builder() -> #bident {
            #bident {
                    #(#default_field_values,)*
                          }
            }
        }
        impl #bident {
            #(#methods)*

         pub fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {

    Ok (
        #name {
            #(#non_option_iter: self.#non_option_iter.clone().ok_or("missing field")?,)*
            #(#option_iter: self.#option_iter.clone(),)*
        }
    )

             }

        }

    };
    gen.into()
}
