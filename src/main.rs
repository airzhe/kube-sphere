pub use self::error::{Error, Result};
use axum::middleware;
use axum::{routing::get, Router};
use kube::Client;
use log::*;
use std::net::SocketAddr;
use std::sync::Arc;

mod error;
mod models;
mod services;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = Arc::new(Client::try_default().await.unwrap());

    info!("starting up");

    let k8s_api = Router::new()
        .merge(web::namespaces::routes(client.clone()))
        .merge(web::pods::routes(client.clone()))
        .merge(web::deployments::routes(client.clone()))
        .merge(web::configmaps::routes(client.clone()))
        .merge(web::ingress::routes(client.clone()));

    let app = Router::new();
    let routes_all = Router::new()
        .merge(routes_ping())
        .nest("/api/v1", k8s_api)
        .merge(app)
        .layer(middleware::from_fn_with_state(
            client.clone(),
            web::my_middleware,
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}

fn routes_ping() -> Router {
    Router::new().route("/ping", get(|| async { "pong" }))
}
