use crate::{Error, Result};
use anyhow::anyhow;
use handlebars::Handlebars;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{ConfigMap, Secret, Service};
use kube::api::{DeleteParams, Patch, PatchParams, PostParams};
use kube::{api::Api, Client};
use log::*;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "src/web/templates/"]
struct TemplateDirectory;

#[derive(Serialize, Deserialize)]
struct ProcessResult {
    success: Vec<String>,
    failure: Vec<String>,
}

pub(crate) async fn create_deployment(
    client: Arc<Client>,
    namespace: &str,
    service_name: &str,
    data: serde_json::Value,
) -> Result<String> {
    let mut handlebars = Handlebars::new();
    let template_name = format!("{}.yaml.hbs", service_name);
    let template_file = TemplateDirectory::get(&template_name).ok_or(Error::TemplateNotFound)?;
    let template_str =
        std::str::from_utf8(&template_file.data).map_err(|e| Error::General(e.into()))?;
    info!("{}", template_str);
    handlebars
        .register_template_string(service_name, template_str)
        .map_err(|e| Error::General(e.into()))?;

    let rendered = handlebars
        .render(service_name, &data)
        .map_err(|e| Error::General(e.into()))?;
    info!("{}", rendered);

    let mut success_results = Vec::new();
    let mut failure_results = Vec::new();

    //把doc放到map里，然后再便利map执行创建资源操作试试
    let mut container: Vec<serde_yaml::Value> = Vec::new();

    for document in serde_yaml::Deserializer::from_str(&rendered) {
        let doc = serde_yaml::Value::deserialize(document).map_err(|e| Error::General(e.into()))?;
        container.push(doc);
    }

    info!("Container length: {}", container.len());
    for doc in container {
        // 处理每个doc
        if !doc.is_null() {
            match process_resource(&doc, namespace, client.as_ref()).await {
                Ok(msg) => {
                    success_results.push(msg);
                }
                Err(err) => {
                    failure_results.push(err.to_string());
                }
            }
        }
    }

    let result = ProcessResult {
        success: success_results,
        failure: failure_results,
    };

    let json_result = serde_json::to_string(&result).map_err(|e| Error::General(e.into()))?;

    Ok(json_result)
}

async fn process_resource(
    doc: &serde_yaml::Value,
    namespace: &str,
    client: &Client,
) -> Result<String> {
    let resource_type = doc["kind"]
        .as_str()
        .ok_or_else(|| Error::General(anyhow!("Missing 'kind' field")))?;
    match resource_type {
        "Deployment" => {
            let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
            let deployment = serde_yaml::from_value::<Deployment>(doc.clone())
                .map_err(|e| Error::General(e.into()))?;
            let pp = PostParams::default();
            match api.create(&pp, &deployment).await {
                Ok(resource) => Ok(format!(
                    "Created Deployment: {}",
                    resource.metadata.name.unwrap()
                )),
                Err(kube::Error::Api(ae)) => {
                    if ae.code == 409 {
                        Err(Error::ResourceAlreadyExists(anyhow!(
                            "Resource {} {} already exists",
                            resource_type,
                            deployment.metadata.name.unwrap()
                        )))
                    } else {
                        Err(Error::General(anyhow::Error::new(ae)))
                    }
                }
                Err(e) => Err(e.into()),
            }
        }
        "Service" => {
            let api: Api<Service> = Api::namespaced(client.clone(), namespace);
            let service: Service = serde_yaml::from_value::<Service>(doc.clone())
                .map_err(|e| Error::General(e.into()))?;
            let pp = PostParams::default();
            match api.create(&pp, &service).await {
                Ok(resource) => Ok(format!(
                    "Created Service: {}",
                    resource.metadata.name.unwrap()
                )),
                Err(kube::Error::Api(ae)) => {
                    if ae.code == 409 {
                        Err(Error::ResourceAlreadyExists(anyhow!(
                            "Resource {} {} already exists",
                            resource_type,
                            service.metadata.name.unwrap()
                        )))
                    } else {
                        Err(Error::General(anyhow::Error::new(ae)))
                    }
                }
                Err(e) => Err(e.into()),
            }
        }
        "ConfigMap" => {
            let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
            let config_map = serde_yaml::from_value::<ConfigMap>(doc.clone())
                .map_err(|e| Error::General(e.into()))?;
            let pp = PostParams::default();
            match api.create(&pp, &config_map).await {
                Ok(resource) => Ok(format!(
                    "Created ConfigMap: {}",
                    resource.metadata.name.unwrap()
                )),
                Err(e) => Err(e.into()),
            }
        }
        "Secret" => {
            let api: Api<Secret> = Api::namespaced(client.clone(), namespace);
            let config_map = serde_yaml::from_value::<Secret>(doc.clone())
                .map_err(|e| Error::General(e.into()))?;
            let pp = PostParams::default();
            match api.create(&pp, &config_map).await {
                Ok(resource) => Ok(format!(
                    "Created Secret: {}",
                    resource.metadata.name.unwrap()
                )),
                Err(e) => Err(e.into()),
            }
        }
        _ => Err(Error::UnsupportedKind),
    }
}

pub async fn get_deployment(
    client: Arc<Client>,
    namespace: &str,
    deployment_name: &str,
) -> Result<String> {
    let deployment_api: Api<Deployment> = Api::namespaced(client.as_ref().clone(), namespace);
    match deployment_api.get(deployment_name).await {
        Ok(deployment) => {
            let status = deployment.status.as_ref().ok_or(Error::ResourceNotFound)?;
            let available_replicas = status.available_replicas.unwrap_or_default();
            let replicas = status.replicas.unwrap_or_default();
            Ok(format!(
                "Deployment {} has {} available out of {} replicas",
                deployment_name, available_replicas, replicas
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub(crate) async fn patch_deployment(
    client: Arc<Client>,
    namespace: &str,
    deployment_name: &str,
    data: serde_json::Value,
) -> Result<String> {
    let deployment_api: Api<Deployment> = Api::namespaced(client.as_ref().clone(), &namespace);
    match deployment_api.get(&deployment_name).await {
        Ok(_) => {
            info!(
                "deployment {} was found in namespace {}",
                deployment_name, namespace
            );
            // 发送 Patch 请求来更新 Deployment
            let patch_params = PatchParams::default();
            match deployment_api
                .patch(&deployment_name, &patch_params, &Patch::Merge(&data))
                .await
            {
                Ok(_updated_deployment) => {
                    info!("Deployment updated: {}", deployment_name);
                    Ok("OK".to_owned())
                }
                Err(e) => Err(e.into()),
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub(crate) async fn delete_deployment(
    client: Arc<Client>,
    namespace: &str,
    deployment_name: &str,
) -> Result<String> {
    let deployment_api: Api<Deployment> = Api::namespaced(client.as_ref().clone(), &namespace);
    // 创建 Delete 请求参数
    let delete_params = DeleteParams::default();
    // 发送 Delete 请求来删除 Deployment
    match deployment_api
        .delete(&deployment_name, &delete_params)
        .await
    {
        Ok(_) => {
            info!("Deployment deleted: {}", deployment_name);
            Ok("OK".to_owned())
        }
        Err(err) => Err(err.into()),
    }
}
