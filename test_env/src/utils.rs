use casper_types::{account::AccountHash, ContractPackageHash};

pub(crate) struct OdraAddressWrapper(pub odra::types::Address);

impl From<AccountHash> for OdraAddressWrapper {
    fn from(hash: AccountHash) -> Self {
        OdraAddressWrapper(odra::types::Address::new(hash.as_bytes()))
    }
}

impl From<ContractPackageHash> for OdraAddressWrapper {
    fn from(hash: ContractPackageHash) -> Self {
        OdraAddressWrapper(odra::types::Address::new(hash.as_bytes()))
    }
}

impl Into<ContractPackageHash> for OdraAddressWrapper {
    fn into(self) -> ContractPackageHash {
        let mut bytes_vec = self.0.bytes().to_vec();
        bytes_vec.resize(32, 0);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(bytes_vec.as_slice());

        ContractPackageHash::new(bytes)
    }
}

#[cfg(test)]
mod tests {
    use casper_commons::address::Address as CasperAddress;
    use casper_types::{ContractPackageHash};
    use crate::utils::OdraAddressWrapper;

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
