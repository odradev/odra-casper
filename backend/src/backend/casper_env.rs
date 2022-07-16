use std::collections::BTreeSet;

use casper_commons::address::Address;
use casper_contract::{
    contract_api::{self, runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use odra::types::{
    api_error,
    bytesrepr::{Bytes, FromBytes, ToBytes},
    contracts::NamedKeys,
    system::CallStackElement,
    ApiError, CLTyped, CLValue, ContractPackageHash, ContractVersion, EntryPoints, RuntimeArgs,
    URef,
};

/// Save value to the storage.
pub fn set_cl_value(name: &str, value: CLValue) {
    let bytes: Bytes = value.to_bytes().unwrap_or_revert().into();
    set_key(name, bytes);
}

/// Read value from the storage.
pub fn get_cl_value(name: &str) -> Option<CLValue> {
    get_key::<Bytes>(name).map(|bytes| {
        let (result, _rest) = CLValue::from_bytes(&bytes).unwrap_or_revert();
        result
    })
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
        }
    }
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session
            // wants to interact, caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

fn take_call_stack_elem(n: usize) -> CallStackElement {
    runtime::get_call_stack()
        .into_iter()
        .nth_back(n)
        .unwrap_or_revert()
}

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that only session code can execute this function, and disallows stored
/// session/stored contracts.
pub fn caller() -> Address {
    let second_elem = take_call_stack_elem(1);
    call_stack_element_to_address(second_elem)
}

/// Gets the address of the currently run contract
pub fn self_address() -> Address {
    let first_elem = take_call_stack_elem(0);
    call_stack_element_to_address(first_elem)
}

/// Record event to the contract's storage.
pub fn emit<T: ToBytes>(event: T) {
    // Events::default().emit(event);
}

/// Convert any key to hash.
pub fn to_dictionary_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap_or_revert();
    let bytes = runtime::blake2b(preimage);
    hex::encode(bytes)
}

/// Calls a contract method by Address
pub fn call_contract(address: Address, entry_point: &str, runtime_args: RuntimeArgs) -> Vec<u8> {
    let contract_package_hash = address.as_contract_package_hash().unwrap_or_revert();
    let contract_version: Option<ContractVersion> = None;

    let (contract_package_hash_ptr, contract_package_hash_size, _bytes) =
        to_ptr(*contract_package_hash);
    let (contract_version_ptr, contract_version_size, _bytes) = to_ptr(contract_version);
    let (entry_point_name_ptr, entry_point_name_size, _bytes) = to_ptr(entry_point);
    let (runtime_args_ptr, runtime_args_size, _bytes) = to_ptr(runtime_args);

    let bytes_written = {
        let mut bytes_written = std::mem::MaybeUninit::uninit();
        let ret = unsafe {
            casper_contract::ext_ffi::casper_call_versioned_contract(
                contract_package_hash_ptr,
                contract_package_hash_size,
                contract_version_ptr,
                contract_version_size,
                entry_point_name_ptr,
                entry_point_name_size,
                runtime_args_ptr,
                runtime_args_size,
                bytes_written.as_mut_ptr(),
            )
        };
        api_error::result_from(ret).unwrap_or_revert();
        unsafe { bytes_written.assume_init() }
    };

    let serialized_result = if bytes_written == 0 {
        // If no bytes were written, the host buffer hasn't been set and hence shouldn't be read.
        vec![]
    } else {
        // NOTE: this is a copy of the contents of `read_host_buffer()`.  Calling that directly from
        // here causes several contracts to fail with a Wasmi `Unreachable` error.
        let bytes_non_null_ptr = contract_api::alloc_bytes(bytes_written);
        let mut dest: Vec<u8> = unsafe {
            Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), bytes_written, bytes_written)
        };

        read_host_buffer_into(&mut dest).unwrap_or_revert();
        dest
    };
    serialized_result
}

pub fn install_contract(
    package_hash: &str,
    entry_points: EntryPoints,
    initializer: impl FnOnce(ContractPackageHash),
) {
    // Create a new contract package hash for the contract.
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    runtime::put_key(package_hash, contract_package_hash.into());
    storage::add_contract_version(contract_package_hash, entry_points, NamedKeys::new());

    let init_access: URef =
        storage::create_contract_user_group(contract_package_hash, "init", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call contrustor method.
    initializer(contract_package_hash);

    // Revoke access to init.
    let mut urefs = BTreeSet::new();
    urefs.insert(init_access);
    storage::remove_contract_user_group_urefs(contract_package_hash, "init", urefs)
        .unwrap_or_revert();
}

pub fn get_block_time() -> u64 {
    u64::from(runtime::get_blocktime())
}

pub fn revert(error: u16) -> ! {
    runtime::revert(ApiError::User(error))
}

pub fn print(message: &str) {
    runtime::print(message)
}

fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}

fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
    let mut bytes_written = std::mem::MaybeUninit::uninit();
    let ret = unsafe {
        casper_contract::ext_ffi::casper_read_host_buffer(
            dest.as_mut_ptr(),
            dest.len(),
            bytes_written.as_mut_ptr(),
        )
    };
    // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
    // caller ignores the return value, execution of the contract becomes unstable and ultimately
    // leads to `Unreachable` error.
    api_error::result_from(ret)?;
    Ok(unsafe { bytes_written.assume_init() })
}
