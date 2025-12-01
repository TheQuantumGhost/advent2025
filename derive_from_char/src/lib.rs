use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod fromchar;

#[proc_macro_derive(FromChar, attributes(c))]
pub fn derive_macro_fromchar(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    fromchar::derive_proc_macro_impl(&input).unwrap_or_else(|err| {
        let name = &input.ident;
        let dummy = quote::quote! {
            impl FromChar for #name {
                type Err = ();
                fn from_char(c: char) -> Result<Self, Self::Err> {
                    unimplemented!()
                }
            }
        };
        to_compile_error(err, dummy.into()).into()
    })
}

fn to_compile_error(
    error: syn::Error,
    dummy: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let compile_errors = error.to_compile_error();
    quote::quote!(
        #dummy
        #compile_errors
    )
}
