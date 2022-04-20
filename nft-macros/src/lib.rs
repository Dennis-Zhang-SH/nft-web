use proc_macro::TokenStream; // no need to import a specific crate for TokenStream
use quote::quote;

// Generate a compile error to output struct name
#[proc_macro_derive(CommonResponse)]
pub fn derive_common_response(tokens: TokenStream) -> TokenStream {
    // convert the input tokens into an ast, specially from a derive
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    let name = &ast.ident;

    quote! {
        impl Into<CommonResponse<User>> for #name {
            fn into(self) -> CommonResponse<#name> {
                CommonResponse::Ok(Json(Message { data: self }))
            }
        }
    }
    .into()
}
