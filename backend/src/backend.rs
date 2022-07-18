mod casper_env;
use lazy_static::lazy_static;

use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::Mutex,
};
pub use casper_commons::{odra_address_wrapper::OdraAddressWrapper, address::Address};
pub use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
use odra::types::{URef, Key, Address as OdraAddress, CLValue, ContractPackageHash, RuntimeArgs, EventData, OdraError};

#[no_mangle]
pub fn __get_blocktime() -> u64 {
    casper_env::get_block_time()
}

#[no_mangle]
pub fn __caller() -> OdraAddress {
    casper_env::caller().into()
}

#[no_mangle]
pub fn __set_var(key: &[u8], value: &CLValue) {
    let name = std::str::from_utf8(key).unwrap();
    casper_env::set_cl_value(name, value.clone());
}

#[no_mangle]
fn __get_var(key: &[u8]) -> Option<CLValue> {
    let name = std::str::from_utf8(key).unwrap();
    casper_env::get_cl_value(name)
}

#[no_mangle]
fn __set_dict_value(dict: &[u8], key: &[u8], value: &CLValue) {
    let dict = std::str::from_utf8(dict).unwrap();
    casper_env::set_dict_value(dict, key, value);
}

#[no_mangle]
fn __get_dict_value(dict: &[u8], key: &[u8]) -> Option<CLValue> {
    let dict = std::str::from_utf8(dict).unwrap();
    casper_env::get_dict_value(dict, key)
}

#[no_mangle]
fn __revert(reason: &OdraError) -> ! {
    let code = match reason {
        OdraError::ExecutionError(code, _) => *code,
        _ => 0
    };
    casper_env::revert(code);
}

#[no_mangle]
fn __print(message: &str) {
    casper_env::print(message);
}

#[no_mangle]
pub fn __call_contract(address: &OdraAddress, entrypoint: &str, args: &RuntimeArgs) -> Vec<u8> {
    let address: Address = OdraAddressWrapper::new(*address).into(); 
    casper_env::call_contract(address, entrypoint, args.clone())
}

#[no_mangle]
fn __emit_event(event: &EventData) {
    casper_env::emit_event(event);
}

// @TODO: rename to 
pub fn is_named_arg_exist(name: &str) -> bool {
    let mut arg_size: usize = 0;
    let ret = unsafe {
        casper_contract::ext_ffi::casper_get_named_arg_size(
            name.as_bytes().as_ptr(),
            name.len(),
            &mut arg_size as *mut usize,
        )
    };
    odra::types::api_error::result_from(ret).is_ok()
}
