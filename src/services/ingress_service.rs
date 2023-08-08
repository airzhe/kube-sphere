use crate::{Error, Result};
use k8s_openapi::api::networking::v1::Ingress;
use kube::api::{Api, DeleteParams};
use kube::Client;
use std::sync::Arc;

pub async fn delete(client: Arc<Client>, namespace: &str, name: &str) -> Result<String> {
    let ingress_api: Api<Ingress> = Api::namespaced(client.as_ref().clone(), namespace);
    let dp = DeleteParams::default();

    match ingress_api.delete(name, &dp).await {
        Ok(_) => Ok(format!("delete ingress: {}", name)),
        Err(e) => Err(Error::General(e.into())),
    }
}
