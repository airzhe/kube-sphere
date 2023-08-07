use crate::models::pod;
use crate::{Error, Result};
use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::api::DeleteParams;
use kube::{
    api::{AttachParams, AttachedProcess, ListParams, LogParams},
    Api, Client,
};
use serde_json::json;

use log::*;
use std::sync::Arc;

pub(crate) async fn pod_info(
    client: Arc<Client>,
    namespace: &str,
    pod_name: &str,
) -> Result<String> {
    let pod_api: Api<Pod> = Api::namespaced(client.as_ref().clone(), &namespace);
    let pod = pod_api.get(&pod_name).await.map_err(Error::from)?;
    info!("Pod {} was found in namespace {}", pod_name, namespace);
    let pod_spec_json = serde_json::to_string(&pod.spec.unwrap()).map_err(Error::from)?;
    Ok(pod_spec_json)
}

pub(crate) async fn pod_logs(
    client: Arc<Client>,
    namespace: &str,
    pod_name: &str,
) -> Result<String> {
    let pod_api: Api<Pod> = Api::namespaced(client.as_ref().clone(), &namespace);
    let logs = pod_api
        .logs(
            pod_name,
            &LogParams {
                follow: false,
                since_seconds: Some(10),
                ..LogParams::default()
            },
        )
        .await?;
    Ok(logs.to_owned())
}

pub(crate) async fn exec(
    client: Arc<Client>,
    namespace: &str,
    pod_name: &str,
    exec_params: pod::ExecParams,
) -> String {
    let pod_api: Api<Pod> = Api::namespaced(client.as_ref().clone(), &namespace);

    let attach_params = AttachParams {
        container: exec_params.container,
        ..Default::default()
    };
    info!("exec command {:?} in {}", exec_params.commands, pod_name);
    let attached = pod_api
        .exec(&pod_name, exec_params.commands, &attach_params)
        .await
        .unwrap();
    let output = get_output(attached).await;
    info!("{output}");
    output
}

async fn get_output(mut attached: AttachedProcess) -> String {
    let stdout = tokio_util::io::ReaderStream::new(attached.stdout().unwrap());
    let out = stdout
        .filter_map(|r| async { r.ok().and_then(|v| String::from_utf8(v.to_vec()).ok()) })
        .collect::<Vec<_>>()
        .await
        .join("");
    attached.join().await.unwrap();
    out
}

pub(crate) async fn find_pod_by_labels(
    client: Arc<Client>,
    namespace: &str,
    labels: &str,
) -> Result<String> {
    let pod_api: Api<Pod> = Api::namespaced(client.as_ref().clone(), &namespace);
    let lp = ListParams::default().labels(&labels); // Filter pods by label

    let pods = pod_api.list(&lp).await.map_err(Error::from)?;
    let pods_json = pods
        .iter()
        .map(|pod| {
            let pod_name = &pod.metadata.name;
            let pod_namespace = &pod.metadata.namespace;
            let pod_labels = &pod.metadata.labels;

            json!({
                "name": pod_name,
                "namespace": pod_namespace,
                "labels": pod_labels,
            })
        })
        .collect::<Vec<_>>();

    let result = json!(pods_json);
    Ok(result.to_string())
}

pub(crate) async fn del_pod(
    client: Arc<Client>,
    namespace: &str,
    pod_name: &str,
) -> Result<String> {
    let pod_api: Api<Pod> = Api::namespaced(client.as_ref().clone(), &namespace);
    let delete_params = DeleteParams::default();
    let delete_result = pod_api.delete(pod_name, &delete_params).await?;
    // delete_params
    Ok(format!("Pod delete result: {:?}", delete_result))
}
