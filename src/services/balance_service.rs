use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address,utils::format_units};
use eyre::{Result};
use std::env;

pub async fn get_balance_on_chain(_address: String, _token: String) -> Result<String> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = format!("https://mainnet.infura.io/v3/{}", env::var("INFURA_KEY")?).parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get ETH balance
    if let "ETH" = &*_token {
        let addr = Address::parse_checksummed(_address, None).unwrap();
        let balance = provider.get_balance(addr).await?;
        let formatted_balance = format_units(balance, "ether").map_err(|e| eyre::eyre!("Error formatting balance: {:?}", e))?;
        return Ok(formatted_balance);
    }

    Ok("0".to_string())
}