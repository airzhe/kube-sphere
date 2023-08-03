//Deployment 作为 Kubernetes 中最常见和重要的资源类型之一，通常是指建立服务集合的最常见方式之一。
//并且与其他资源类型（如 Service 和 Ingress）一起使用的。
use crate::services::*;
use crate::{Result};
use axum::{
    extract::{Json, Path, State},
    routing::get,
    Router,
};
use kube::Client;
use std::sync::Arc;

pub fn routes(client: Arc<Client>) -> Router {
    Router::new()
        .route(
            "/namespaces/:namespace/deployments/:deployment_name",
            get(get_deployment)
                .patch(patch_deployment)
                .post(create_deployment)
                .delete(delete_deployment),
        )
        .with_state(client)
}

async fn create_deployment(
    State(client): State<Arc<Client>>,
    Path((namespace, deployment_name)): Path<(String, String)>,
    Json(data): Json<serde_json::Value>,
) -> Result<String> {
    deployment_service::create_deployment(client, &namespace, &deployment_name, data).await
}

pub async fn get_deployment(
    State(client): State<Arc<Client>>,
    Path((namespace, deployment_name)): Path<(String, String)>,
) -> Result<String> {
    deployment_service::get_deployment(client, &namespace, &deployment_name).await
}

async fn patch_deployment(
    State(client): State<Arc<Client>>,
    Path((namespace, deployment_name)): Path<(String, String)>,
    Json(data): Json<serde_json::Value>,
) -> Result<String> {
    deployment_service::patch_deployment(client, &namespace, &deployment_name, data).await
}

async fn delete_deployment(
    State(client): State<Arc<Client>>,
    Path((namespace, deployment_name)): Path<(String, String)>,
) -> Result<String> {
    deployment_service::delete_deployment(client, &namespace, &deployment_name).await
}