use axum::{extract::State, http::Request, middleware::Next, response::Response};
use kube::Client;
use std::sync::Arc;
pub mod deployments;
pub mod pods;
pub mod namespaces;
pub mod configmaps;

pub(crate) async fn my_middleware<B>(
    State(_client): State<Arc<Client>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // do something with `request`...
    let response = next.run(request).await;
    // do something with `response`...
    response
}
