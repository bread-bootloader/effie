use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn w(input: TokenStream) -> TokenStream {
    let lit: LitStr = parse_macro_input!(input);

    let encoded = lit.value().encode_utf16().collect::<Vec<u16>>();

    quote! {
        unsafe {::effie::WStr::from_bytes(&[#( #encoded, )* 0u16])}
    }
    .into()
}

#[proc_macro]
#[doc(hidden)]
pub fn w_internal(input: TokenStream) -> TokenStream {
    let lit: LitStr = parse_macro_input!(input);

    let encoded = lit.value().encode_utf16().collect::<Vec<u16>>();

    quote! {
       unsafe { crate::WStr::from_bytes(&[#( #encoded, )* 0u16])}
    }
    .into()
}
