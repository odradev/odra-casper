mod casper_env;

pub use casper_contract;
use odra::types::{Address as OdraAddress, CLValue};

#[no_mangle]
pub fn get_blocktime() -> u64 {
    casper_env::get_block_time()
}

#[no_mangle]
fn caller() -> OdraAddress {
    casper_env::caller().into()
}

#[no_mangle]
fn set_var(key: &[u8], value: &CLValue) {
    let name = std::str::from_utf8(key).unwrap();
    casper_env::set_cl_value(name, value.clone());
}

#[no_mangle]
fn get_var(key: &[u8]) -> Option<CLValue> {
    let name = std::str::from_utf8(key).unwrap();
    casper_env::get_cl_value(name)
}
