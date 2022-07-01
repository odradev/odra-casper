use odra::contract_def::Argument;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

pub(super) struct CasperArgs<'a>(pub &'a Vec<Argument>);

impl ToTokens for CasperArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.0;

        args.iter().for_each(|arg| {
            let arg_ident = format_ident!("{}", arg.ident);

            tokens.append_all(quote! {
                let #arg_ident = casper_backend::backend::casper_contract::contract_api::runtime::get_named_arg(stringify!(#arg_ident));
            });
        });
    }
}
