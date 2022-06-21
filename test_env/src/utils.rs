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