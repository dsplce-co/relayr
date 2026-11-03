use proc_macro::{TokenStream};
use quote::quote;
use syn::Ident;
use syn::{parse_macro_input, punctuated::Punctuated, Lit, Token, ItemFn};
use syn::parse::{Parse, Parser};

fn extract_first_arg<T: Parse>(input: TokenStream) -> Option<T> {
    Punctuated::<T, Token![,]>::parse_terminated
        .parse(input.clone())
        .ok()
        .and_then(|args| args.into_iter().next())
}

fn split_patterns(input: &str) -> Vec<String> {
    input
        .split(", ")
        .filter_map(|pattern| {
            let pattern = pattern.trim();
            (!pattern.is_empty()).then(|| pattern.to_string())
        })
        .collect()
}

fn extract_cron_expression(input: TokenStream) -> proc_macro2::TokenStream {
    if let Some(Lit::Str(cron_literal)) = extract_first_arg::<Lit>(input.clone()) {
        let cron_literal_string = cron_literal.value();

        let patterns = split_patterns(&cron_literal_string);

        if patterns.is_empty() {
            panic!("Expected at least one cron expression");
        }

        for pattern in patterns {
            croner::Cron::new(&pattern)
                .with_seconds_optional()
                .parse()
                .unwrap_or_else(|err| panic!("{pattern:?} doesn't seem to be a valid cron expression: {err}"));
        }

        return quote!(relayr::CronPattern::Lit(#cron_literal_string));
    }

    if let Some(environment_variable) = extract_first_arg::<Ident>(input) {
        return quote!(relayr::CronPattern::EnvVar(stringify!(#environment_variable)));
    }

    panic!("Expected cron literal expression or environment variable name");
}

#[proc_macro_attribute]
pub fn cron(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cron_expression = extract_cron_expression(attr);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let result = quote! {
        #input

        relayr::inventory::submit! {
            relayr::Cron { pattern: #cron_expression, runnable: |job_id| Box::pin(#fn_name(job_id)), name: stringify!(#fn_name) }
        }
    };

    result.into()
}
