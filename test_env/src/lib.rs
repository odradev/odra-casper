use crate::env::ENV;
use casper_types::{bytesrepr::Bytes, RuntimeArgs, ContractPackageHash};
use odra::types::Address as OdraAddress;
use utils::OdraAddressWrapper;

pub mod env;
mod utils;

#[no_mangle]
fn backend_name() -> String {
    "Casper test backend".to_string()
}

#[no_mangle]
fn register_contract(name: &str, args: &RuntimeArgs) -> OdraAddress {
    ENV.with(|env| {
        env.borrow_mut()
            .deploy_contract(&format!("{}.wasm", name), args.clone());

        let contract_package_hash = format!("{}_package_hash", name);
        let wrapped_address: OdraAddressWrapper = env
            .borrow()
            .get_contract_package_hash(&contract_package_hash)
            .into();
        *wrapped_address
    })
}

#[no_mangle]
pub fn call_contract(
    addr: &OdraAddress,
    entrypoint: &str,
    args: &RuntimeArgs,
    has_return: bool,
) -> Option<Bytes> {
    ENV.with(|env| {
        let contract_hash: ContractPackageHash = OdraAddressWrapper::new(addr.to_owned()).into();
        env.borrow_mut()
            .call_contract(contract_hash, entrypoint, args.to_owned(), has_return)
    })
}

#[no_mangle]
pub fn assert_exception<F, E>(err: E, block: F)
where
    F: Fn() -> (),
    E: Into<OdraError>,
{
    todo!();
    // block();
    // let exec_err = borrow_env()
    //     .error()
    //     .expect("An error expected, but did not occur");
    // assert_eq!(exec_err, err.into());
}

#[no_mangle]
pub fn set_caller(address: &OdraAddress) {
    ENV.with(|env| {
        let odra_address = OdraAddressWrapper::new(address.to_owned());
        env.borrow_mut().as_account(odra_address.into())
    })
}

#[no_mangle]
pub fn get_account(n: usize) -> OdraAddress {
    ENV.with(|env| {
        env.borrow().get_account(n).into()
    })
}