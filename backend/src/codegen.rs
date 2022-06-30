use convert_case::Casing;
use odra::contract_def::ContractDef;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};

use self::{call::ContractEntrypoints, entrypoints::WasmEntrypoint};

mod call;
mod entrypoints;

pub fn gen_contract(contract_def: ContractDef) -> TokenStream2 {
    let entrypoints = generate_entrypoints(&contract_def);
    let call_fn = generate_call(&contract_def);

    quote! {
        #![no_main]

        use odra::instance::Instance;
        use casper_backend;

        #call_fn

        #entrypoints
    }
}

fn generate_entrypoints(contract_def: &ContractDef) -> TokenStream2 {
    let contract_ident = format_ident!("{}", contract_def.ident);

    contract_def
        .entrypoints
        .iter()
        .map(|ep| WasmEntrypoint(&ep, &contract_ident).to_token_stream())
        .flatten()
        .collect::<TokenStream2>()
}

fn generate_call(contract_def: &ContractDef) -> TokenStream2 {
    let entrypoints = ContractEntrypoints(&contract_def.entrypoints).to_token_stream();
    let package_hash =
        format!("{}_package_hash", &contract_def.ident).to_case(convert_case::Case::Snake);

    quote! {
        #[no_mangle]
        fn call() {
            let (contract_package_hash, _) = casper_backend::backend::casper_contract::contract_api::storage::create_contract_package_at_hash();
            casper_backend::backend::casper_contract::contract_api::runtime::put_key(#package_hash, contract_package_hash.into());

            #entrypoints

            casper_backend::backend::casper_contract::contract_api::storage::add_contract_version(
                contract_package_hash,
                entry_points,
                odra::types::contracts::NamedKeys::new()
            );
        }
    }
}
