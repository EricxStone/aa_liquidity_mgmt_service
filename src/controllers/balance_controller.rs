use axum::{extract::Query};
use serde::Deserialize;
use crate::services::balance_service;


#[derive(Deserialize)]
pub struct GetBalanceParameters {
    address: String,
    token: String,
}

pub async fn get_balance(Query(params): Query<GetBalanceParameters>) -> String {
    let balance = balance_service::get_balance_on_chain(params.address, params.token);
    balance.await.unwrap()
}