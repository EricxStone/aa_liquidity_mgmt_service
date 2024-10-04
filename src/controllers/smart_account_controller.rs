use axum::extract::{
        Query,
        Json,
};
use serde::Deserialize;
use crate::services::alchemy_light_account;

#[derive(Deserialize)]
pub struct GetAddressParameters {
    owner: String,
    ref_id: String,
}
#[derive(Deserialize)]
pub struct TransferEthParameters {
    owner: String,
    ref_id: String,
    to: String,
    amount: String,
}

pub async fn get_address(Query(params): Query<GetAddressParameters>) -> String {
    let account_address = alchemy_light_account::get_account(params.owner, params.ref_id);
    account_address.await.unwrap()
}

pub async fn transfer_eth(Json(payload): Json<TransferEthParameters>) -> String {
    let transfer_result = alchemy_light_account::transfer_eth(payload.owner, payload.ref_id, payload.to, payload.amount);
    transfer_result.await.unwrap()
}