use std::{cell::RefCell, path::PathBuf};

use casper_commons::address::Address;
use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
    DEFAULT_PAYMENT,
};
use casper_execution_engine::core::engine_state::{
    run_genesis_request::RunGenesisRequest, GenesisAccount,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes, ToBytes},
    runtime_args, CLTyped, ContractPackageHash, Key, Motes, PublicKey, RuntimeArgs, SecretKey,
    U512,
};

thread_local! {
    pub static ENV: RefCell<CasperTestEnv> = RefCell::new(CasperTestEnv::new());
}

pub struct CasperTestEnv {
    accounts: Vec<Address>,
    active_account: Address,
    context: InMemoryWasmTestBuilder,
    block_time: u64,
    calls_counter: u32,
}

impl CasperTestEnv {
    pub fn new() -> Self {
        let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
        let mut accounts: Vec<Address> = Vec::new();
        for i in 0..20 {
            // Create keypair.
            let secret_key = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
            let public_key = PublicKey::from(&secret_key);

            // Create an AccountHash from a public key.
            let account_addr = AccountHash::from(&public_key);

            // Create a GenesisAccount.
            let account = GenesisAccount::account(
                public_key,
                Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
                None,
            );
            genesis_config.ee_config_mut().push_account(account);

            accounts.push(account_addr.into());
        }
        let run_genesis_request = RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            genesis_config.protocol_version(),
            genesis_config.take_ee_config(),
        );

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&run_genesis_request).commit();

        Self {
            active_account: accounts[0].clone(),
            context: builder,
            accounts,
            block_time: 0,
            calls_counter: 0,
        }
    }

    pub fn deploy_contract(&mut self, wasm_path: &str, args: RuntimeArgs) {
        let session_code = PathBuf::from(wasm_path);
        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[self.active_account_hash()])
            .with_address(self.active_account_hash())
            .with_session_code(session_code, args)
            .with_deploy_hash(self.next_hash())
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item)
            .with_block_time(self.block_time)
            .build();
        self.context.exec(execute_request).commit().expect_success();
        self.active_account = self.get_account(0);
    }

    pub fn call_contract(
        &mut self,
        hash: ContractPackageHash,
        entry_point: &str,
        args: RuntimeArgs,
        has_return: bool,
    ) -> Option<Bytes> {
        let session_code = PathBuf::from("getter_proxy.wasm");

        let args_bytes: Vec<u8> = args.to_bytes().unwrap();
        let args = runtime_args! {
            "contract_package_hash" => hash,
            "entry_point" => entry_point,
            "args" => Bytes::from(args_bytes),
            "has_return" => has_return
        };

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[self.active_account_hash()])
            .with_address(self.active_account_hash())
            .with_session_code(session_code, args)
            .with_deploy_hash(self.next_hash())
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item)
            .with_block_time(self.block_time)
            .build();
        self.context.exec(execute_request).commit().expect_success();

        let active_account = self.active_account_hash();

        let result = if has_return {
            let result: Bytes = self.get_account_value(active_account, "result");
            Some(result)
        } else {
            None
        };
        self.active_account = self.get_account(0);
        result
    }

    pub fn set_caller(&mut self, account: Address) {
        self.active_account = account;
    }

    fn active_account_hash(&self) -> AccountHash {
        *self.active_account.as_account_hash().unwrap()
    }

    fn get_account(&self, n: usize) -> Address {
        *self.accounts.get(n).unwrap()
    }

    fn as_account(&mut self, account: Address) {
        self.active_account = account;
    }

    fn next_hash(&mut self) -> [u8; 32] {
        let seed = self.calls_counter;
        self.calls_counter += 1;
        let mut hash = [0u8; 32];
        hash[0] = seed as u8;
        hash[1] = (seed >> 8) as u8;
        hash
    }

    pub fn get_account_value<T: FromBytes + CLTyped>(&self, hash: AccountHash, name: &str) -> T {
        self.context
            .query(None, Key::Account(hash), &[name.to_string()])
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }

    pub fn get_contract_package_hash(&self, name: &str) -> ContractPackageHash {
        let account = self
            .context
            .get_account(self.active_account_hash())
            .unwrap();
        let key: &Key = account.named_keys().get(name).unwrap();
        ContractPackageHash::from(key.into_hash().unwrap())
    }
}
