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
                let result = casper_backend::backend::casper_types::CLValue::from_t(result).unwrap_or_revert();
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

#[cfg(test)]
mod tests {
    use crate::codegen::assert_eq_tokens;
    use odra::contract_def::{Argument, EntrypointType};
    use odra::types::CLType;

    use super::*;

    #[test]
    fn test_constructor() {
        let entrypoint = Entrypoint {
            ident: String::from("construct_me"),
            args: vec![Argument {
                ident: String::from("value"),
                ty: CLType::I32,
            }],
            ret: CLType::Unit,
            ty: EntrypointType::Public,
        };
        let path: Path = syn::parse2(
            quote! {
                my_contract::MyContract
            }
            .to_token_stream(),
        )
        .unwrap();

        let wasm_entrypoint = WasmEntrypoint(&entrypoint, &path);
        assert_eq_tokens(
            wasm_entrypoint,
            quote!(
                #[no_mangle]
                fn construct_me() {
                    let contract = my_contract::MyContract::instance("contract");
                    let value = casper_backend::backend::casper_contract::contract_api::runtime::get_named_arg(stringify!(value));
                    contract.construct_me(value);
                }
            ),
        );
    }
}
