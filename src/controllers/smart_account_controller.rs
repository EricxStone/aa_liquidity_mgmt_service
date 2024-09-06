use axum::{extract::Query};
use serde::Deserialize;
use crate::services::alchemy_light_account;

#[derive(Deserialize)]
pub struct GetAddressParameters {
    owner: String,
    ref_id: String,
}

pub async fn get_address(Query(params): Query<GetAddressParameters>) -> String {
    let account_address = alchemy_light_account::get_account(params.owner, params.ref_id);
    account_address.await.unwrap()
}