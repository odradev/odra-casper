use odra::contract_def::Entrypoint;
use quote::{format_ident, quote, ToTokens};
use syn::{self, punctuated::Punctuated, token::Comma, Ident, Path};

use super::arg::CasperArgs;

pub(crate) struct WasmEntrypoint<'a>(pub &'a Entrypoint, pub &'a Path);

impl ToTokens for WasmEntrypoint<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let entrypoint_ident = format_ident!("{}", &self.0.ident);
        let args = CasperArgs(&self.0.args).to_token_stream();

        let mut fn_args = Punctuated::<Ident, Comma>::new();
        self.0
            .args
            .iter()
            .for_each(|arg| fn_args.push(format_ident!("{}", arg.ident)));

        let contract_call = match self.0.ret {
            odra::types::CLType::Unit => quote! {
                #args
                contract.#entrypoint_ident(#fn_args);
            },
            _ => quote! {
                use casper_backend::backend::casper_contract::unwrap_or_revert::UnwrapOrRevert;
                #args
                let result = contract.#entrypoint_ident(#fn_args);
                let result = odra::types::CLValue::from_t(result).unwrap_or_revert();
                casper_backend::backend::casper_contract::contract_api::runtime::ret(result);
            },
        };

        let contract_path = &self.1;

        tokens.extend(quote! {
            #[no_mangle]
            fn #entrypoint_ident() {
                let contract = #contract_path::instance("contract");
                #contract_call
            }
        });
    }
}
