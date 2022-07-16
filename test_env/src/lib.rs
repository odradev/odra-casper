use crate::env::ENV;
use casper_types::{bytesrepr::Bytes, RuntimeArgs, ContractPackageHash};
use odra::types::{Address as OdraAddress, OdraError};
use casper_commons::{address::Address as CasperAddress, odra_address_wrapper::OdraAddressWrapper};

pub mod env;

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
        let contract_package_hash = env.borrow().get_contract_package_hash(&contract_package_hash);
        let casper_address: CasperAddress = contract_package_hash.into();
        let wrapped_address: OdraAddressWrapper = casper_address.into();
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
        let contract_hash: CasperAddress = OdraAddressWrapper::new(addr.to_owned()).into();
        let contract_hash = contract_hash.as_contract_package_hash().unwrap();
        env.borrow_mut()
            .call_contract(*contract_hash, entrypoint, args.to_owned(), has_return)
    })
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

#[no_mangle]
pub fn get_error() -> Option<OdraError> {
    ENV.with(|env| {
        env.borrow().get_error()
    })
}