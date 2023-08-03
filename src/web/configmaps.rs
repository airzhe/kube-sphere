use crate::{services::configmap_service, Result};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use kube::Client;
use std::{collections::BTreeMap, sync::Arc};

pub fn routes(client: Arc<Client>) -> Router {
    Router::new()
        .route(
            "/namespaces/:namespace/configmaps/:configmap",
            get(configmap_info)
                .put(update_configmap)
                .post(create_configmap)
                .delete(delete_configmap),
        )
        .with_state(client)
}

async fn create_configmap(
    State(client): State<Arc<Client>>,
    Path((namespace, configmap)): Path<(String, String)>,
    Json(payload): Json<BTreeMap<String, String>>,
) -> Result<String> {
    configmap_service::create(client, &namespace, &configmap, payload).await
}

async fn configmap_info(
    State(client): State<Arc<Client>>,
    Path((namespace, configmap)): Path<(String, String)>,
) -> Result<String> {
    configmap_service::get(client, &namespace, &configmap).await
}

async fn update_configmap(
    State(client): State<Arc<Client>>,
    Path((namespace, configmap)): Path<(String, String)>,
    Json(payload): Json<BTreeMap<String, String>>,
) -> Result<String> {
    configmap_service::update(client, &namespace, &configmap, payload).await
}

async fn delete_configmap(
    State(client): State<Arc<Client>>,
    Path((namespace, configmap)): Path<(String, String)>,
) -> Result<String> {
    configmap_service::delete(client, &namespace, &configmap).await
}