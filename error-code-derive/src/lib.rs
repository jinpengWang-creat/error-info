#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident, Type};

#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    process_to_error_info(input)
}

use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromField, FromMeta, FromMetaItem, FromVariant,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_info))]
struct ErrorInfoEnumDarling {
    ident: Ident,
    generics: Generics,
    data: Data<ErrorInfoVariant, ()>,
    app_type: Type,
    prefix: String,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(error_info))]
struct ErrorInfoVariant {
    ident: Ident,
    fields: Fields<Ignored>,
    code: String,
    #[darling(default)]
    app_code: String,
    #[darling(default)]
    client_msg: String,
}

fn process_to_error_info(input: DeriveInput) -> TokenStream {
    let ErrorInfoEnumDarling {
        ident,
        generics,
        data: Data::Enum(vars),
        app_type,
        prefix,
    } = ErrorInfoEnumDarling::from_derive_input(&input).unwrap()
    else {
        panic!("Only enum is supported")
    };

    let codes: Vec<_> = vars
        .iter()
        .map(|var| {
            let ErrorInfoVariant {
                ident: var,
                fields: _,
                code,
                app_code,
                client_msg,
            } = var;
            let code = format!("{}{}", prefix, code);
            quote! {
                #ident::#var(_) => ErrorInfo::try_new(#app_code, #code, #client_msg, format!("{}", self))
            }
        })
        .collect();
    quote! {
        use error_code::{ErrorInfo, ToErrorInfo as _};
        impl #generics ToErrorInfo for #ident #generics {
            type T = #app_type;

            fn to_error_info(&self) -> Result<ErrorInfo<Self::T>, <Self::T as std::str::FromStr>::Err> {
                match self {
                    #(#codes),*
                }
            }
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use darling::error;

    #[test]
    fn test_darling_data_struct() -> Result<()> {
        let input = r##"
            #[derive(Error, Debug, ToErrorInfo)]
            #[error_info(app_type = "StatusCode", prefix = "01")]
            pub enum MyError {
                #[error("Invalid command: {0}")]
                #[error_info(code = "IC", app_code = "400")]
                InvalidCommand(String),

                #[error("Invalid argument: {0}")]
                #[error_info(code = "IA", app_code = "400", client_msg = "friendly msg")]
                InvalidArgument(String),
                #[error("{0}")]
                #[error_info(code = "RE", app_code = "500")]
                RespError(#[from] FromUtf8Error),
            }
        "##;
        let parsed = syn::parse_str(input)?;
        let error_info = ErrorInfoEnumDarling::from_derive_input(&parsed).unwrap();
        println!("{:#?}", error_info);
        Ok(())
    }
}
