use self::{call::ContractEntrypoints, constructor::WasmConstructor, entrypoints::WasmEntrypoint};
use odra::contract_def::{ContractDef, EntrypointType};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, Path, PathSegment, Token};

mod arg;
mod call;
mod constructor;
mod entrypoints;
mod ty;

// TODO: Put those functions into trait inside odra, so each backend will implement them
pub fn gen_contract(contract_def: ContractDef, fqn: String) -> TokenStream2 {
    let entrypoints = generate_entrypoints(&contract_def, fqn.clone());
    let call_fn = generate_call(&contract_def, fqn + "Ref");

    quote! {
        #![no_main]

        use odra::instance::Instance;
        use casper_backend;

        #call_fn

        #entrypoints
    }
}

fn generate_entrypoints(contract_def: &ContractDef, fqn: String) -> TokenStream2 {
    let path = &fqn_to_path(fqn);
    contract_def
        .entrypoints
        .iter()
        .flat_map(|ep| WasmEntrypoint(ep, path).to_token_stream())
        .collect::<TokenStream2>()
}

fn generate_call(contract_def: &ContractDef, ref_fqn: String) -> TokenStream2 {
    let entrypoints = ContractEntrypoints(&contract_def.entrypoints);
    let contract_def_name_snake = odra::utils::camel_to_snake(&contract_def.ident);
    let package_hash = format!("{}_package_hash", contract_def_name_snake);

    let constructors = contract_def
        .entrypoints
        .iter()
        .filter(|ep| ep.ty == EntrypointType::Constructor)
        .collect::<Vec<_>>();

    let ref_path = &fqn_to_path(ref_fqn);
    let call_constructor = WasmConstructor(constructors, ref_path);

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

fn fqn_to_path(fqn: String) -> Path {
    let paths = fqn.split("::").collect::<Vec<_>>();

    let segments = Punctuated::<PathSegment, Token![::]>::from_iter(
        paths
            .iter()
            .map(|ident| PathSegment::from(format_ident!("{}", ident))),
    );

    syn::Path {
        leading_colon: None,
        segments,
    }
}
