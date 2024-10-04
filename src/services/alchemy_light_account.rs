use std::str::FromStr;
use std::env;
use alloy::{
    primitives::{Address, U256, aliases::U192 as U192, address, bytes, Bytes, utils::parse_units},
    providers::{ProviderBuilder, ext::Erc4337Api},
    sol,
    sol_types::SolCall,
};
use alloy_rpc_types_eth::{
    SendUserOperation,
    UserOperationGasEstimation
};
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use alloy_transport::TransportResult;
use crate::services::alchemy_light_account::EntryPoint_0_7::PackedUserOperation;
use eyre::Result;
use serde::{Deserialize, Serialize};

const LIGHT_ACCOUNT_V2_FACTORY: Address = address!("0000000000400CdFef5E2714E63d8040b700BC24");
const ENTRY_POINT_0_7: Address = address!("0000000071727De22E5E9d8BAf0edAc6f37da032");
const LIGHT_ACCOUNT_V2_DUMMY_SIGNATURE: Bytes = bytes!("00fffffffffffffffffffffffffffffff0000000000000000000000000000000007aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1c");

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

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EntryPoint_0_7,
    "src/abi/EntryPoint_0_7.json"
);

sol!(
    #[allow(missing_docs)]
    function execute(
        address dest,
        uint256 value,
        bytes calldata func,
    ) external;
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOperationForGasEstimation{
    sender: Address,
    nonce: U256,
    factory: Address,
    factory_data: Bytes,
    call_data: Bytes,
    paymaster: Option<Address>,
    paymaster_data: Option<Bytes>,
    signature: Bytes,
}

pub async fn get_account(owner: String, salt: String) -> Result<String> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = format!("https://eth-sepolia.g.alchemy.com/v2/{}", env::var("ALCHEMY_KEY")?).parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let contract = LightAccountFactory::new(LIGHT_ACCOUNT_V2_FACTORY, provider);
    let account = contract.getAddress(Address::parse_checksummed(owner, None).unwrap(), U256::from_str(salt.as_str()).unwrap()).call().await?._0;

    Ok(account.to_string())
}

fn get_account_init_code(owner: String, salt: String) -> Bytes {
    let owner = Address::parse_checksummed(owner, None).unwrap();
    let salt = U256::from_str(salt.as_str()).unwrap();
    let init_code_data = createAccountCall::new((owner, salt));
    Bytes::from(createAccountCall::abi_encode(&init_code_data))
}

pub async fn transfer_eth(owner: String, salt: String, to: String, amount: String) -> Result<String> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let bundler_url = format!("https://eth-sepolia.g.alchemy.com/v2/{}", env::var("ALCHEMY_KEY")?).parse()?;

    // Prepare user operation for gas estimation
    let account = get_account(owner.clone(), salt.clone()).await.unwrap();
    let account_init_code = get_account_init_code(owner.clone(), salt.clone());
    let call_data = get_eth_transfer_call_data(to.clone(), amount.clone());
    let user_operation_for_estimation = prepare_user_operations_for_estimation(account, account_init_code, call_data).await;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let client: ReqwestClient = ClientBuilder::default().http(bundler_url);
    let gas_estimation_result: TransportResult<UserOperationGasEstimation>  = client.request("eth_estimateUserOperationGas", (user_operation_for_estimation, ENTRY_POINT_0_7)).await;

    print!("Gas estimation result: {:?}", gas_estimation_result.unwrap());

    

    Ok("0".to_string())
}


async fn get_nonce(account: String) -> Result<U256> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = format!("https://eth-sepolia.g.alchemy.com/v2/{}", env::var("ALCHEMY_KEY")?).parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let account = Address::parse_checksummed(account, None).unwrap();
    let entry_point_contract = EntryPoint_0_7::new(ENTRY_POINT_0_7, provider);
    let key = U192::from_str("0")?;
    let nonce = entry_point_contract.getNonce(account, key).call().await?.nonce;
    Ok(nonce)
}

fn get_eth_transfer_call_data(to: String, amount: String) -> Bytes {
    let to = Address::parse_checksummed(to, None).unwrap();
    let wei_amount = parse_units(&amount, "wei").unwrap().into();
    let func_data = bytes!("");
    let call_data = executeCall::new((to, wei_amount, func_data));
    Bytes::from(call_data.abi_encode())
}

async fn prepare_user_operations_for_estimation(account: String, account_init_code: Bytes, call_data: Bytes) -> UserOperationForGasEstimation {
    let nonce = get_nonce(account.clone()).await.unwrap();
    let sender = Address::parse_checksummed(account.clone(), None).unwrap();
    let factory = LIGHT_ACCOUNT_V2_FACTORY;    
    let dummy_signature = LIGHT_ACCOUNT_V2_DUMMY_SIGNATURE;
    UserOperationForGasEstimation {
        sender,
        nonce,
        factory,
        factory_data: account_init_code,
        call_data,
        signature: dummy_signature,
        paymaster: None,
        paymaster_data: None,
    }
}