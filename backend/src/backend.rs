pub use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
use odra::types::{Address as OdraAddress, CLValue, EventData, ExecutionError, RuntimeArgs};
pub use odra_casper_shared::casper_address::CasperAddress;

mod casper_env;

#[no_mangle]
pub fn __get_blocktime() -> u64 {
    casper_env::get_block_time()
}

#[no_mangle]
pub fn __caller() -> OdraAddress {
    OdraAddress::try_from(casper_env::caller()).unwrap()
}

#[no_mangle]
pub fn __self_address() -> OdraAddress {
    OdraAddress::try_from(casper_env::self_address()).unwrap()
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
fn __revert(reason: &ExecutionError) -> ! {
    casper_env::revert(reason.code());
}

#[no_mangle]
fn __print(message: &str) {
    casper_env::print(message);
}

#[no_mangle]
pub fn __call_contract(address: &OdraAddress, entrypoint: &str, args: &RuntimeArgs) -> Vec<u8> {
    let casper_address = CasperAddress::try_from(*address).unwrap();
    casper_env::call_contract(casper_address, entrypoint, args.clone())
}

#[no_mangle]
fn __emit_event(event: &EventData) {
    casper_env::emit_event(event);
}

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
