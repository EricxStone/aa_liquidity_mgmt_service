use axum::Router;
use dotenv::dotenv;

mod routes;
mod controllers;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let app = Router::new()
        .nest("/balance", routes::api_routes::get_balance_routes())
        .nest("/account", routes::api_routes::get_address_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

