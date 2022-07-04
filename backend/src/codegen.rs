use convert_case::Casing;
use odra::contract_def::{ContractDef, EntrypointType};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use std::path::Path;
use syn::punctuated::Punctuated;
use syn::{parse_quote, PathSegment, Token};

use self::{call::ContractEntrypoints, constructor::WasmConstructor, entrypoints::WasmEntrypoint};

mod arg;
mod call;
mod constructor;
mod entrypoints;
mod ty;

// TODO: Put those functions into trait inside odra, so each backend will implement them

pub fn gen_contract(contract_def: ContractDef, fqn: String) -> TokenStream2 {
    let entrypoints = generate_entrypoints(&contract_def, fqn);
    let call_fn = generate_call(&contract_def);

    quote! {
        #![no_main]

        use odra::instance::Instance;
        use casper_backend;

        #call_fn

        #entrypoints
    }
}

fn generate_entrypoints(contract_def: &ContractDef, fqn: String) -> TokenStream2 {
    let paths = fqn.split("::").collect::<Vec<_>>();

    let mut segments: Punctuated<PathSegment, Token![::]> = Punctuated::new();
    paths.iter().for_each(|p| {
        segments.push(PathSegment {
            ident: format_ident!("{}", p),
            arguments: syn::PathArguments::None,
        });
    });

    let path = syn::Path {
        leading_colon: None,
        segments,
    };

    contract_def
        .entrypoints
        .iter()
        .map(|ep| WasmEntrypoint(&ep, &path).to_token_stream())
        .flatten()
        .collect::<TokenStream2>()
}

fn generate_call(contract_def: &ContractDef) -> TokenStream2 {
    let entrypoints = ContractEntrypoints(&contract_def.entrypoints);
    let package_hash =
        format!("{}_package_hash", &contract_def.ident).to_case(convert_case::Case::Snake);

    let constructors = contract_def
        .entrypoints
        .iter()
        .filter(|ep| ep.ty == EntrypointType::Constructor)
        .collect::<Vec<_>>();

    let call_constructor = WasmConstructor(constructors, &contract_def.ident);

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

            #call_constructor
        }
    }
}
