use crate::{Error, Result};
use k8s_openapi::api::core::v1::Namespace;
use kube::api::{Api, DeleteParams, PostParams};
use kube::core::ObjectMeta;
use kube::{api::ListParams, Client};
use std::sync::Arc;

pub async fn list(client: Arc<Client>) -> Result<axum::Json<Vec<String>>> {
    let namespaces: Api<Namespace> = Api::all(client.as_ref().clone());

    let lp = ListParams::default();
    let nss = namespaces
        .list(&lp)
        .await
        .map_err(|e| Error::General(e.into()))?;

    let namespace_names: Vec<String> = nss
        .iter()
        .filter_map(|ns| ns.metadata.name.clone())
        .collect();

    Ok(axum::Json(namespace_names))
}

pub async fn create(client: Arc<Client>, name: &str) -> Result<String> {
    let namespaces: Api<Namespace> = Api::all(client.as_ref().clone());
    let pp = PostParams::default();

    let ns = &Namespace {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    match namespaces.create(&pp, ns).await {
        Ok(_) => Ok(format!("Created namespace: {}", name)),
        Err(e) => Err(Error::General(e.into())),
    }
}

pub async fn delete(client: Arc<Client>, name: &str) -> Result<String> {
    let namespaces: Api<Namespace> = Api::all(client.as_ref().clone());
    let dp = DeleteParams::default();

    match namespaces.delete(name, &dp).await {
        Ok(_) => Ok(format!("delete namespace: {}", name)),
        Err(e) => Err(Error::General(e.into())),
    }
}
