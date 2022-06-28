mod casper_env;

use casper_commons::address::Address;
pub use casper_contract;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use odra::types::{Address as OdraAddress, CLValue, RuntimeArgs, CLType, U256, ContractPackageHash};

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
pub fn __call_contract(
    address: &OdraAddress,
    entrypoint: &str,
    args: &RuntimeArgs,
    returned_type: &CLType,
) -> CLValue { 
    let s = format!("odra addr: {:?}", address);
    casper_env::print(&s);
    
    let address = Address::Contract(ContractPackageHash::try_from(&address.data).unwrap());
    let s = format!("backend addr: {:?}", address);
    casper_env::print(&s);
    let res = match returned_type {
        CLType::Bool => {
            let value = casper_env::call_contract::<bool>(address, entrypoint, args.clone());
            CLValue::from_t(value).unwrap_or_revert()
        },
        CLType::I32 => {
            let value = casper_env::call_contract::<i32>(address, entrypoint, args.clone());
            CLValue::from_t(value).unwrap_or_revert()
        }
        CLType::U32 => {
            __print("dupau32");
            let value = casper_env::call_contract::<u32>(address, entrypoint, args.clone());
            __print("dupa");
            CLValue::from_t(value).unwrap_or_revert()
        },
        CLType::U256 => {
            let value = casper_env::call_contract::<U256>(address, entrypoint, args.clone());
            CLValue::from_t(value).unwrap_or_revert()
        },
        CLType::Unit => {
            let value = casper_env::call_contract::<()>(address, entrypoint, args.clone());
            CLValue::from_t(value).unwrap_or_revert()
        },
        CLType::String => {
            let value = casper_env::call_contract::<String>(address, entrypoint, args.clone());
            CLValue::from_t(value).unwrap_or_revert()
        },
        _ => __revert(888)
    };
    let s = format!("{:?}", res);
    __print(&s);
    res
    // let res = if has_return {
    //  let a: Bytes = casper_env::call_contract(*address, entrypoint, *args);
    // }
    // None
}
