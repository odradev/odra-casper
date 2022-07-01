use odra::contract_def::Entrypoint;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident};

use super::arg::CasperArgs;

pub(crate) struct WasmConstructor<'a>(pub &'a Entrypoint, pub &'a String);

impl ToTokens for WasmConstructor<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let entrypoint_ident = format_ident!("{}", &self.0.ident);
        let args = CasperArgs(&self.0.args);

        let mut fn_args = Punctuated::<Ident, Comma>::new();
        self.0
            .args
            .iter()
            .for_each(|arg| fn_args.push(format_ident!("{}", arg.ident)));

        let ref_ident = format_ident!("{}Ref", &self.1);

        tokens.extend(quote! {
            use casper_backend::backend::casper_contract::unwrap_or_revert::UnwrapOrRevert;
            let constructor_access: odra::types::URef =
                casper_backend::backend::casper_contract::contract_api::storage::create_contract_user_group(
                    contract_package_hash,
                    "constructor",
                    1,
                    Default::default()
                )
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

            let casper_address = casper_backend::backend::Address::from(contract_package_hash);
            let odra_address: odra::types::Address = casper_address.into();

            let contract_ref = sample_contract::#ref_ident {
                address: odra_address,
            };
            #args

            contract_ref.#entrypoint_ident( #fn_args );

            // Revoke access to constructor.
            let mut urefs = std::collections::BTreeSet::new();
            urefs.insert(constructor_access);
            casper_backend::backend::casper_contract::contract_api::storage::remove_contract_user_group_urefs(
                contract_package_hash,
                "constructor",
                urefs
            ).unwrap_or_revert();
            });
    }
}
