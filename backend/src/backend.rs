//! Casper backend for WASM.
//!
//! It provides all the required functions to communicate between Odra and Casper.
use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
pub use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
pub use casper_types;
use odra::types::{Address as OdraAddress, CLValue, EventData, ExecutionError, RuntimeArgs};
pub use odra_casper_shared::casper_address::CasperAddress;

use crate::casper_env;

/// Returns blocktime.
#[no_mangle]
fn __get_blocktime() -> u64 {
    casper_env::get_block_time()
}

/// Returns contract caller.
#[no_mangle]
pub fn __caller() -> OdraAddress {
    OdraAddress::try_from(casper_env::caller()).unwrap_or_revert()
}

/// Returns current contract address.
#[no_mangle]
pub fn __self_address() -> OdraAddress {
    OdraAddress::try_from(casper_env::self_address()).unwrap_or_revert()
}

/// Store a value into the storage.
#[no_mangle]
pub fn __set_var(key: &str, value: &CLValue) {
    casper_env::set_cl_value(key, value.clone());
}

/// Read value from the storage.
#[no_mangle]
pub fn __get_var(key: &str) -> Option<CLValue> {
    casper_env::get_cl_value(key)
}

/// Store the mapping value under a given key.
#[no_mangle]
pub fn __set_dict_value(dict: &str, key: &[u8], value: &CLValue) {
    casper_env::set_dict_value(dict, key, value);
}

/// Read value from the mapping.
#[no_mangle]
pub fn __get_dict_value(dict: &str, key: &[u8]) -> Option<CLValue> {
    casper_env::get_dict_value(dict, key)
}

/// Revert the execution.
#[no_mangle]
pub fn __revert(reason: &ExecutionError) -> ! {
    casper_env::revert(reason.code());
}

// #[no_mangle]
// fn __print(message: &str) {
//     casper_env::print(message);
// }

/// Call another contract.
#[no_mangle]
pub fn __call_contract(address: &OdraAddress, entrypoint: &str, args: &RuntimeArgs) -> Vec<u8> {
    let casper_address = CasperAddress::try_from(*address).unwrap_or_revert();
    casper_env::call_contract(casper_address, entrypoint, args.clone())
}

/// Emit event.
#[no_mangle]
pub fn __emit_event(event: &EventData) {
    casper_env::emit_event(event);
}

/// Check if given named argument exists.
pub fn named_arg_exists(name: &str) -> bool {
    let mut arg_size: usize = 0;
    let ret = unsafe {
        casper_contract::ext_ffi::casper_get_named_arg_size(
            name.as_bytes().as_ptr(),
            name.len(),
            &mut arg_size as *mut usize,
        )
    };
    casper_types::api_error::result_from(ret).is_ok()
}
