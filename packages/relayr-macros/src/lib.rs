use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{Expr, ExprLit, ExprPath};
use syn::{parse_macro_input, punctuated::Punctuated, Lit, Token, ItemFn};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn cron(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse attribute args");

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let pattern_expression = args.first().expect("Expected cron expression");

    let cron_expression = match pattern_expression {
        Expr::Lit(ExprLit { lit: Lit::Str(cron_literal), .. }) => {
            let cron_literal_string = cron_literal.value();
            croner::Cron::new(&cron_literal_string)
                .with_seconds_optional()
                .parse()
                .expect("It doesn't seem to be a valid cron expression");

            quote!(relayr::CronPattern::Literal(#cron_literal_string))
        },
        Expr::Path(ExprPath { path, .. }) if path.segments.len() > 0 => {
            let variable_name_string = path.to_token_stream().to_string();

            quote!(relayr::CronPattern::EnvironmentVariable(#variable_name_string))
        },
        _ => panic!("Expected string literal or environment variable identifier, e.g. #[cron(\"...\")] or #[cron(ENV_VARIABLE)]"),
    };

    let result = quote! {
        #input

        relayr::inventory::submit! {
            relayr::Cron { pattern: #cron_expression, runnable: |job_id| Box::pin(#fn_name(job_id)), name: stringify!(#fn_name) }
        }
    };

    result.into()
}
