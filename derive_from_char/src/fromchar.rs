use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, Variant};

pub(crate) fn derive_proc_macro_impl(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let DeriveInput {
        ident: name, data, ..
    } = input;

    let tagged_idents = match data {
        syn::Data::Enum(enum_data) => {
            //eprintln!("{:#?}", enumData);
            enum_data
                .variants
                .iter()
                .map(|variant| parse_variant(variant))
                .collect::<Vec<_>>()
        }
        _ => panic!("Throw compile error"),
    };

    let match_arms = tagged_idents
        .iter()
        .map(|(variant, chars)| {
            quote::quote! {
                #(#chars)|* => Ok(Self::#variant),
            }
        })
        .collect::<Vec<_>>();

    let impl_block = quote::quote! {
        impl utilities::FromChar for #name {
            type Err = utilities::ParseError;
            fn from_char(c: char) -> Result<Self, Self::Err> {
                match c {
                    #(#match_arms)*
                    _ => Err(Self::Err::WrongChar(c)),
                }
            }
        }
    };
    Ok(impl_block.into())
}

fn parse_variant(variant: &Variant) -> (Ident, Vec<char>) {
    let name = variant.ident.clone();

    let chars = variant
        .attrs
        .iter()
        .filter_map(check_attribute)
        .collect::<Vec<_>>();
    (name, chars)
}

fn check_attribute(attribute: &syn::Attribute) -> Option<char> {
    if attribute.style != syn::AttrStyle::Outer {
        return None;
    }
    if let syn::Meta::NameValue(value) = &attribute.meta {
        let is_c_attribute = value.path.segments.iter().collect::<Vec<_>>()[0].ident == "c";
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Char(l),
            ..
        }) = &value.value
        {
            return (is_c_attribute).then(|| l.value());
        }
    }

    None
}
