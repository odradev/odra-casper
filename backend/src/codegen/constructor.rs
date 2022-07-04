use odra::contract_def::Entrypoint;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Ident};

use super::arg::CasperArgs;
type FnArgs = Punctuated<Ident, Comma>;

pub(crate) struct WasmConstructor<'a>(pub Vec<&'a Entrypoint>, pub &'a String);

impl ToTokens for WasmConstructor<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let data: Vec<(Ident, CasperArgs, FnArgs)> = self
            .0
            .iter()
            .map(|ep| {
                let entrypoint_ident = format_ident!("{}", &ep.ident);
                let casper_args = CasperArgs(&ep.args);

                let mut fn_args = Punctuated::<Ident, Comma>::new();
                ep.args
                    .iter()
                    .for_each(|arg| fn_args.push(format_ident!("{}", arg.ident)));

                (entrypoint_ident, casper_args, fn_args)
            })
            .collect();

        let ref_ident = format_ident!("{}Ref", &self.1);
        let constructor_matching: proc_macro2::TokenStream = data
            .iter()
            .map(|(entrypoint_ident, casper_args, fn_args)| {
                quote! {
                    stringify!(#entrypoint_ident) => {
                        let contract_ref = sample_contract::#ref_ident {
                            address: odra_address,
                        };
                        #casper_args

                        contract_ref.#entrypoint_ident( #fn_args );
                    },
                }
            })
            .flatten()
            .collect();

        tokens.extend(quote! {
            if casper_backend::backend::is_named_arg_exist("constructor") {
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

                let constructor_name = casper_backend::backend::casper_contract::contract_api::runtime::get_named_arg::<String>(
                    "constructor",
                );
                let constructor_name = constructor_name.as_str();

                match constructor_name {
                    #constructor_matching
                    _ => {}
                };

                // Revoke access to constructor.
                let mut urefs = std::collections::BTreeSet::new();
                urefs.insert(constructor_access);
                casper_backend::backend::casper_contract::contract_api::storage::remove_contract_user_group_urefs(
                    contract_package_hash,
                    "constructor",
                    urefs
                ).unwrap_or_revert();
            }
        });
    }
}
