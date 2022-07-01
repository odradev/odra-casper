mod casper_env;

pub use casper_commons::address::Address;
pub use casper_contract;
use odra::types::{Address as OdraAddress, CLValue, ContractPackageHash, RuntimeArgs};

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
fn __revert(reason: u32) -> ! {
    casper_env::revert(reason)
}

#[no_mangle]
fn __print(message: &str) {
    casper_env::print(message);
}

#[no_mangle]
pub fn __call_contract(address: &OdraAddress, entrypoint: &str, args: &RuntimeArgs) -> Vec<u8> {
    let address = Address::Contract(ContractPackageHash::try_from(address.bytes()).unwrap());
    casper_env::call_contract(address, entrypoint, args.clone())
}

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
