use crate::models::pod;
use crate::{services::pod_service, Result};
use axum::extract::Query;
use axum::{
    extract::{Json, Path, State},
    routing::{get, post},
    Router,
};
use kube::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct QueryParams {
    labels: Option<String>,
}

pub fn routes(client: Arc<Client>) -> Router {
    Router::new()
        .route(
            "/namespaces/:namespace/pods/:pod_name",
            get(pod_info).delete(del_pod),
        )
        .route("/namespaces/:namespace/pods/:pod_name/exec", post(execute))
        .route("/namespaces/:namespace/pods/:pod_name/logs", get(pod_logs))
        .route("/namespaces/:namespace/pods", get(find_pod_by_labels))
        .with_state(client)
}

async fn pod_info(
    State(client): State<Arc<Client>>,
    Path((namespace, pod_name)): Path<(String, String)>,
) -> Result<String> {
    pod_service::pod_info(client, &namespace, &pod_name).await
}

async fn del_pod(
    State(client): State<Arc<Client>>,
    Path((namespace, pod_name)): Path<(String, String)>,
) -> Result<String> {
    pod_service::del_pod(client, &namespace, &pod_name).await
}

async fn pod_logs(
    State(client): State<Arc<Client>>,
    Path((namespace, pod_name)): Path<(String, String)>,
) -> Result<String> {
    pod_service::pod_logs(client, &namespace, &pod_name).await
}

async fn execute(
    State(client): State<Arc<Client>>,
    Path((namespace, pod_name)): Path<(String, String)>,
    Json(exec_params): Json<pod::ExecParams>,
) -> String {
    pod_service::exec(client, &namespace, &pod_name, exec_params).await
}

async fn find_pod_by_labels(
    State(client): State<Arc<Client>>,
    Path(namespace): Path<String>,
    Query(query_params): Query<QueryParams>,
) -> Result<String> {
    let labels = query_params.labels.unwrap_or_default();
    pod_service::find_pod_by_labels(client, &namespace, &labels).await
}
