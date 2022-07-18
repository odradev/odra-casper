use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractPackageHash, Key, 
};

use odra_types::Address as OdraAddress;

/// An enum representing an [`AccountHash`] or a [`ContractPackageHash`].
///
/// It is taken from [`CasperLabs's ERC20`](https://raw.githubusercontent.com/casper-ecosystem/erc20/master/erc20/src/address.rs).
/// It is copied instead of imported for the flexebility.
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CasperAddress {
    /// Represents an account hash.
    Account(AccountHash),
    /// Represents a contract package hash.
    Contract(ContractPackageHash),
}

impl CasperAddress {
    /// Returns the inner account hash if `self` is the `Account` variant.
    pub fn as_account_hash(&self) -> Option<&AccountHash> {
        if let Self::Account(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the inner contract hash if `self` is the `Contract` variant.
    pub fn as_contract_package_hash(&self) -> Option<&ContractPackageHash> {
        if let Self::Contract(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn is_contract(&self) -> bool {
        self.as_contract_package_hash().is_some()
    }
}

impl From<ContractPackageHash> for CasperAddress {
    fn from(contract_package_hash: ContractPackageHash) -> Self {
        Self::Contract(contract_package_hash)
    }
}

impl From<AccountHash> for CasperAddress {
    fn from(account_hash: AccountHash) -> Self {
        Self::Account(account_hash)
    }
}

impl From<CasperAddress> for Key {
    fn from(address: CasperAddress) -> Self {
        match address {
            CasperAddress::Account(account_hash) => Key::Account(account_hash),
            CasperAddress::Contract(contract_package_hash) => Key::Hash(contract_package_hash.value()),
        }
    }
}

impl TryFrom<Key> for CasperAddress {
    type Error = String;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        match key {
            Key::Account(account_hash) => Ok(CasperAddress::Account(account_hash)),
            Key::Hash(contract_package_hash) => Ok(CasperAddress::Contract(ContractPackageHash::new(contract_package_hash))),
            _ => Err(String::from("Unsupport Key type."))
        }
    }
}

impl CLTyped for CasperAddress {
    fn cl_type() -> CLType {
        CLType::Key
    }
}

impl ToBytes for CasperAddress {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        Key::from(*self).to_bytes()
    }

    fn serialized_length(&self) -> usize {
        Key::from(*self).serialized_length()
    }
}

impl FromBytes for CasperAddress {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (key, remainder) = Key::from_bytes(bytes)?;

        let address = match key {
            Key::Account(account_hash) => CasperAddress::Account(account_hash),
            Key::Hash(raw_contract_package_hash) =>
                CasperAddress::Contract(ContractPackageHash::new(raw_contract_package_hash)),
            _ => return Err(bytesrepr::Error::Formatting),
        };

        Ok((address, remainder))
    }
}

impl Into<OdraAddress> for CasperAddress {
    fn into(self) -> OdraAddress {
        OdraAddress::new(&self.to_bytes().unwrap())
    }
}

impl From<&OdraAddress> for CasperAddress {
    fn from(address: &OdraAddress) -> Self {
        let bytes = address.bytes();
        //TODO to add error handling
        <CasperAddress as FromBytes>::from_bytes(bytes).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: casper-types > 1.5.0 will have prefix fixed.
    const CONTRACT_PACKAGE_HASH: &str = "contract-package-wasm7ba9daac84bebee8111c186588f21ebca35550b6cf1244e71768bd871938be6a";
    const ACCOUNT_HASH: &str = "account-hash-3b4ffcfb21411ced5fc1560c3f6ffed86f4885e5ea05cde49d90962a48a14d95";

    fn mock_account_hash() -> AccountHash {
        AccountHash::from_formatted_str(ACCOUNT_HASH).unwrap()
    }

    fn mock_contract_package_hash() -> ContractPackageHash {
        ContractPackageHash::from_formatted_str(CONTRACT_PACKAGE_HASH).unwrap()
    }

    #[test]
    fn test_casper_address_account_hash_conversion() {
        let account_hash = mock_account_hash();
        
        // It is possible to convert CasperAddress back to AccountHash.
        let casper_address = CasperAddress::from(account_hash);
        assert_eq!(casper_address.as_account_hash().unwrap(), &account_hash);

        // It is not possible to convert CasperAddress to ContractPackageHash.
        assert!(casper_address.as_contract_package_hash().is_none());

        // And it is not a contract.
        assert!(!casper_address.is_contract());

        // It can be converted into a Key and back to CasperAddress.
        let key = Key::from(casper_address);
        let restored = CasperAddress::try_from(key);
        assert_eq!(restored.unwrap(), casper_address);

        // It can be converted into bytes and back.
        let bytes = casper_address.to_bytes().unwrap();
    }

    #[test]
    fn test_casper_address_contract_package_hash_conversion() {
        let contract_package_hash = mock_contract_package_hash();
        let casper_address = CasperAddress::from(contract_package_hash);
        
        // It is possible to convert CasperAddress back to ContractPackageHash.
        assert_eq!(casper_address.as_contract_package_hash().unwrap(), &contract_package_hash);

        // It is not possible to convert CasperAddress to AccountHash.
        assert!(casper_address.as_account_hash().is_none());

        // And it is a contract.
        assert!(casper_address.is_contract());

        // It can be converted into a Key and back to CasperAddress.
        let key = Key::from(casper_address);
        let restored = CasperAddress::try_from(key);
        assert_eq!(restored.unwrap(), casper_address);
    }

    #[test]
    fn test_casper_address_key_conversion_fails() {
        let bad_key = Key::SystemContractRegistry;
        assert_eq!(
            CasperAddress::try_from(bad_key),
            Err(String::from("Unsupport Key type."))
        );
    }
}