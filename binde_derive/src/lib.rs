use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Ident, Type};

#[proc_macro_derive(BinaryDeserialize)]
pub fn derive_binary_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match create_binary_deserialize_impl(input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

fn create_binary_deserialize_impl(input: DeriveInput) -> Result<TokenStream, Error> {
    let fields = match input.data {
        Data::Struct(ds) => ds.fields,
        _ => {
            return Err(Error::new_spanned(
                input,
                "only structs may be derived from BinaryDeserialize",
            ))
        }
    };
    let field_types: Vec<&Type> = fields.iter().map(|f| &f.ty).collect();
    let field_names: Vec<&Ident> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let struct_name = input.ident;

    let tokens = quote! {
        impl ::binde::BinaryDeserialize for #struct_name {
            const SIZE: usize = 0 #( + <#field_types as ::binde::BinaryDeserialize>::SIZE )*;

            fn deserialize<E, R>(mut reader: R) -> ::std::io::Result<Self>
            where
                E: ::binde::ByteOrder,
                R: ::std::io::Read,
            {
                Ok(Self {
                    #(
                        #field_names: ::binde::deserialize::<E, _, _>(&mut reader)?
                    ),*
                })
            }
        }
    };
    Ok(tokens.into())
}
