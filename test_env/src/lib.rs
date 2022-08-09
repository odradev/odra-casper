use crate::env::ENV;
use casper_types::{bytesrepr::Bytes, RuntimeArgs};
use odra::types::{event::EventError, Address as OdraAddress, EventData, OdraError};
use odra_casper_shared::casper_address::CasperAddress;

pub mod env;

#[no_mangle]
fn backend_name() -> String {
    "CasperVM".to_string()
}

#[no_mangle]
fn register_contract(name: &str, args: &RuntimeArgs) -> OdraAddress {
    ENV.with(|env| {
        let wasm_name = format!("{}.wasm", name);
        env.borrow_mut().deploy_contract(&wasm_name, args.clone());

        let contract_package_hash = format!("{}_package_hash", name);
        let contract_package_hash = env
            .borrow()
            .get_contract_package_hash(&contract_package_hash);
        let casper_address: CasperAddress = contract_package_hash.into();
        OdraAddress::try_from(casper_address).unwrap()
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
        let casper_address = CasperAddress::try_from(*addr).unwrap();
        let contract_package_hash = casper_address.as_contract_package_hash().unwrap();
        env.borrow_mut().call_contract(
            *contract_package_hash,
            entrypoint,
            args.to_owned(),
            has_return,
        )
    })
}

#[no_mangle]
pub fn set_caller(address: &OdraAddress) {
    ENV.with(|env| {
        let casper_address = CasperAddress::try_from(*address).unwrap();
        env.borrow_mut().as_account(casper_address)
    })
}

#[no_mangle]
pub fn get_account(n: usize) -> OdraAddress {
    ENV.with(|env| OdraAddress::try_from(env.borrow().get_account(n)).unwrap())
}

#[no_mangle]
pub fn get_error() -> Option<OdraError> {
    ENV.with(|env| env.borrow().get_error())
}

#[no_mangle]
pub fn get_event(address: &OdraAddress, index: i32) -> Result<EventData, EventError> {
    ENV.with(|env| {
        let casper_address = CasperAddress::try_from(*address).unwrap();
        env.borrow().get_event(casper_address, index)
    })
}
