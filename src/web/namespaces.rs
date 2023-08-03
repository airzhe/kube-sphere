use crate::models::namespace;
use crate::{services::namespace_service, Result};
use axum::routing::{delete, post};
use axum::Json;
use axum::{
    extract::{Path, State},
    Router,
};
use kube::Client;
use std::sync::Arc;

///namespaces/{namespace}
pub fn routes(client: Arc<Client>) -> Router {
    Router::new()
        .route("/namespaces", post(create_namespace).get(list_namespace))
        .route("/namespaces/:namespace", delete(delete_namespace))
        .with_state(client)
}

async fn list_namespace(State(client): State<Arc<Client>>) -> Result<axum::Json<Vec<String>>> {
    namespace_service::list(client).await
}

async fn create_namespace(
    State(client): State<Arc<Client>>,
    Json(params): Json<namespace::CreateParams>,
) -> Result<String> {
    namespace_service::create(client, &params.name).await
}

async fn delete_namespace(
    State(client): State<Arc<Client>>,
    Path(namespace): Path<String>,
) -> Result<String> {
    namespace_service::delete(client, &namespace).await
}
