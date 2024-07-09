use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as syn::ItemFn);

    if func.sig.asyncness.is_none() {
        return syn::Error::new_spanned(func.sig.fn_token, "function must be async")
            .to_compile_error()
            .into();
    }

    let output = quote::quote! {
        #func
    };

    output.into()
}
