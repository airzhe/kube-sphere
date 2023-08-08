use crate::{services::ingress_service, Result};
use axum::routing::delete;

use axum::{
    extract::{Path, State},
    Router,
};
use kube::Client;
use std::sync::Arc;

pub fn routes(client: Arc<Client>) -> Router {
    Router::new()
        .route(
            "/namespaces/:namespace/ingress/:ingress",
            delete(delete_namespace),
        )
        .with_state(client)
}

async fn delete_namespace(
    State(client): State<Arc<Client>>,
    //Path(namespace, ingress): Path<String, String>,
    Path((namespace, ingress)): Path<(String, String)>,
) -> Result<String> {
    ingress_service::delete(client, &namespace, &ingress).await
}
