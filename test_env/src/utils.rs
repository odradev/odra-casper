use std::ops::Deref;

use casper_commons::address::Address as CasperAddress;
use odra::types::Address as OdraAddress;
use casper_types::{account::AccountHash, ContractPackageHash, bytesrepr::{FromBytes, ToBytes}};

pub(crate) struct OdraAddressWrapper(OdraAddress);

impl OdraAddressWrapper {
    pub fn new(address: OdraAddress) -> Self {
        Self(address)
    }
} 

impl Deref for OdraAddressWrapper {
    type Target = OdraAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<AccountHash> for OdraAddressWrapper {
    fn from(hash: AccountHash) -> Self {
        let casper_address: CasperAddress = hash.into();
        OdraAddressWrapper(casper_address.into())
    }
}

impl From<ContractPackageHash> for OdraAddressWrapper {
    fn from(hash: ContractPackageHash) -> Self {
        let casper_address: CasperAddress = hash.into();
        OdraAddressWrapper(casper_address.into())
    }
}

impl From<CasperAddress> for OdraAddressWrapper {
    fn from(address: CasperAddress) -> Self {
        let bytes = address.to_bytes().unwrap();
        OdraAddressWrapper(OdraAddress::new(bytes.as_slice()))
    }
}

impl Into<CasperAddress> for OdraAddressWrapper {
    fn into(self) -> CasperAddress {
        let vec = self.to_bytes().unwrap();
        CasperAddress::from_vec(vec).unwrap().0
    }
}

impl Into<ContractPackageHash> for OdraAddressWrapper {
    fn into(self) -> ContractPackageHash {
        let mut bytes_vec = self.bytes().to_vec();
        bytes_vec.resize(casper_types::KEY_HASH_LENGTH, 0);
        let mut bytes = [0u8; casper_types::KEY_HASH_LENGTH];
        bytes.copy_from_slice(bytes_vec.as_slice());

        ContractPackageHash::new(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::OdraAddressWrapper;
    use casper_commons::address::Address as CasperAddress;
    use casper_types::ContractPackageHash;

    use odra::types::Address as OdraAddress;

    #[test]
    fn test_address() {
        let casper_addr = ContractPackageHash::new([1u8; 32]);
        let odra_addr = OdraAddressWrapper::from(casper_addr);
        let result: ContractPackageHash = odra_addr.into();
        assert_eq!(result, casper_addr);
    }

    #[test]
    fn test_casper_address_to_odra_address() {
        let casper_addr_ph = ContractPackageHash::new([3u8; 32]);
        let casper_addr = CasperAddress::from(casper_addr_ph);
        let odra_addr: OdraAddress = casper_addr.into();
        let result = CasperAddress::from(&odra_addr);
        assert_eq!(result, casper_addr);
        assert_eq!(result.as_contract_package_hash().unwrap(), &casper_addr_ph);
    }
}
