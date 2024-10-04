
use axum::{routing::{
    get, post
}, Router};
use crate::controllers;

pub fn get_balance_routes() -> Router {
    Router::new().route("/", get(controllers::balance_controller::get_balance))
}

pub fn get_address_routes() -> Router {
    Router::new().route("/", get(controllers::smart_account_controller::get_address))
}

pub fn transfer_eth_routes() -> Router {
    Router::new().route("/", post(controllers::smart_account_controller::transfer_eth))
}
