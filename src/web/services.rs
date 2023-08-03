use axum::{routing::get, Router};
use kube::Client;
use std::sync::Arc;

pub fn routes(client: Arc<Client>) -> Router {
    Router::new()
        //.route("/namespaces/{ns}/pods/{name}", get(pod_info).delete(del_pod))
        .with_state(client)
}

fn delete_server(client: Arc<Client>) -> String {
    String::from("Abc")
}