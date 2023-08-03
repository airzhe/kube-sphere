use crate::{Error, Result};
use k8s_openapi::api::core::v1::ConfigMap;
use kube::api::{Api, DeleteParams, PostParams};
use kube::Client;
use log::info;
use std::collections::BTreeMap;
use std::sync::Arc;

pub async fn create(
    client: Arc<Client>,
    namespace: &str,
    configmap_name: &str,
    data: BTreeMap<String, String>,
) -> Result<String> {
    let configmap_api: Api<ConfigMap> = Api::namespaced(client.as_ref().clone(), namespace);
    let mut new_configmap = ConfigMap::default();
    new_configmap.metadata.name = Some(configmap_name.to_owned());
    new_configmap.data = Some(data);
    let pp = PostParams::default();
    let created_configmap = configmap_api.create(&pp, &new_configmap).await?;
    info!(
        "ConfigMap {} has been created in namespace {}",
        configmap_name, namespace
    );
    Ok(created_configmap.metadata.name.unwrap_or_default())
}

pub async fn get(client: Arc<Client>, namespace: &str, configmap_name: &str) -> Result<String> {
    let configmap_api: Api<ConfigMap> = Api::namespaced(client.as_ref().clone(), namespace);
    let configmap = configmap_api.get(configmap_name).await?;
    info!(
        "ConfigMap {} was found in namespace {}",
        configmap_name, namespace
    );
    let configmap_spec_json = serde_json::to_string(&configmap.data).map_err(Error::from)?;
    Ok(configmap_spec_json)
}

pub async fn update(
    client: Arc<Client>,
    namespace: &str,
    configmap_name: &str,
    data: BTreeMap<String, String>,
) -> Result<String> {
    let configmap_api: Api<ConfigMap> = Api::namespaced(client.as_ref().clone(), namespace);
    let mut configmap = configmap_api.get(configmap_name).await?;
    configmap.data = Some(data);
    let pp = PostParams::default();
    configmap_api
        .replace(configmap_name, &pp, &configmap)
        .await?;
    info!(
        "ConfigMap {} in namespace {} has been updated",
        configmap_name, namespace
    );
    Ok("success".to_owned())
}

pub async fn delete(client: Arc<Client>, namespace: &str, configmap_name: &str) -> Result<String> {
    let configmap_api: Api<ConfigMap> = Api::namespaced(client.as_ref().clone(), namespace);
    let delete_params = DeleteParams::default();
    configmap_api.delete(configmap_name, &delete_params).await?;
    info!(
        "ConfigMap {} has been deleted from namespace {}",
        configmap_name, namespace
    );
    Ok(configmap_name.to_owned())
}
