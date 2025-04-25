use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Lit, Token, ItemFn};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn cron(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = Punctuated::<Lit, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse attribute args");

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let pattern = if let Some(Lit::Str(lit_str)) = args.first() {
        lit_str.value()
    } else {
        panic!("Expected a string literal with a cron expression inside #[cron(\"...\")]");
    };
    
    croner::Cron::new(&pattern)
        .with_seconds_optional()
        .parse()
        .expect("It doesn't seem to be a valid cron expression");

    let result = quote! {
        #input

        inventory::submit! {
            relayr::Cron { pattern: #pattern, runnable: #fn_name }
        }
    };

    result.into()
}
