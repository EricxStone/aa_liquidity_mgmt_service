use std::str::FromStr;
use std::env;
use alloy::{
    hex,
    primitives::{Address, U256, address},
    providers::ProviderBuilder,
    sol,
    sol_types::SolCall,
};
use eyre::Result;

const LIGHT_ACCOUNT_FACTORY: Address = address!("0000000000400CdFef5E2714E63d8040b700BC24");

sol!(
    #[allow(missing_docs)]
    function createAccount(
        address owner,
        uint256 salt
    ) external returns (address ret);
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    LightAccountFactory,
    "src/abi/LightAccountFactory.json"
);

pub async fn get_account(owner: String, salt: String) -> Result<String> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = format!("https://mainnet.infura.io/v3/{}", env::var("INFURA_KEY")?).parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let contract = LightAccountFactory::new(LIGHT_ACCOUNT_FACTORY, provider);
    let account = contract.getAddress(Address::parse_checksummed(owner, None).unwrap(), U256::from_str(salt.as_str()).unwrap()).call().await?._0;

    Ok(account.to_string())
}


fn get_account_init_code(owner: String, salt: String) -> String {
    let owner = Address::parse_checksummed(owner, None).unwrap();
    let salt = U256::from_str(salt.as_str()).unwrap();
    let init_code_data = createAccountCall::new((owner, salt));
    hex::encode(createAccountCall::abi_encode(&init_code_data))
}