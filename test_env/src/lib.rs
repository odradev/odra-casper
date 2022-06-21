use casper_types::RuntimeArgs;
use convert_case::{Casing, Case};
use odra::types::Address;
use odra_test_env::ContractContainer;
use utils::OdraAddressWrapper;

pub mod env;
mod utils;

#[no_mangle]
fn backend_name() -> String {
    "Casper test backend".to_string()
}

#[no_mangle]
fn register_contract(container: &ContractContainer) -> Address {
    crate::env::ENV.with(|env| {
        env.borrow_mut()
            .deploy_contract(container.wasm_path.as_str(), RuntimeArgs::new());

        let contract_package_hash = format!("{}_package_hash", container.name).to_case(Case::Snake);
        let wrapped_address: OdraAddressWrapper = env
            .borrow()
            .get_contract_package_hash(&contract_package_hash)
            .into();
        wrapped_address.0
    })
}

#[no_mangle]
pub fn call_contract(
    addr: &Address,
    entrypoint: &str,
    args: &RuntimeArgs,
    has_return: bool,
) -> Option<Vec<u8>> {
    crate::env::ENV.with(|env| {
        let contract_hash = OdraAddressWrapper(addr.to_owned()).into();
        env.borrow_mut()
            .call_contract(contract_hash, entrypoint, args.to_owned(), has_return)
    })
}
