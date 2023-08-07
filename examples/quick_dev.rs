#![allow(unused)] // For beginning only.
use anyhow::Result;
use chrono::Local;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个 HTTP 客户端
    let hc = httpc_test::new_client("http://localhost:8081")?;

    // 发送 GET 请求到 /ping 路径
    hc.do_get("/ping").await?.print().await?;

    // 创建一个命名空间
    let create_ns: serde_json::Value = json!({
        "name":"kube-rs-test",
    });
    hc.do_post("/api/v1/namespaces", create_ns.clone())
        .await?
        .print()
        .await?;

    // 删除一个命名空间
    let create_ns: serde_json::Value = json!({
        "name":"kube-rs-test",
    });
    hc.do_delete("/api/v1/namespaces/kube-rs-test")
        .await?
        .print()
        .await?;

    // 获取所有命名空间
    hc.do_get("/api/v1/namespaces").await?.print().await?;

    // 创建Configmap
    let configmap_data: serde_json::Value = serde_json::json!({
        "ZEP_OPENAI_API_KEY":"92d7802018eb4ce8bd720049a41cab04"
    });
    hc.do_post(
        "/api/v1/namespaces/beta-popcloud/configmaps/test-configmap",
        configmap_data.clone(),
    )
    .await?
    .print()
    .await?;

    // 修改Configmap
    let configmap_data: serde_json::Value = serde_json::json!({
        "ZEP_OPENAI_API_KEY":"92d7802018eb4ce8bd720049a41cab05"
    });
    hc.do_put(
        "/api/v1/namespaces/beta-popcloud/configmaps/test-configmap",
        configmap_data.clone(),
    )
    .await?
    .print()
    .await?;

    // 获取Configmap
    hc.do_get(
        "/api/v1/namespaces/beta-popcloud/configmaps/test-configmap",
    )
    .await?
    .print()
    .await?;

    // 删除Configmap
    hc.do_delete(
        "/api/v1/namespaces/beta-popcloud/configmaps/test-configmap",
    )
    .await?
    .print()
    .await?;

    // 获取指定命名空间下的 Pod 列表
    hc.do_get("/api/v1/namespaces/beta-popcloud/pods?labels=app.kubernetes.io/name%3Dalpine")
        .await?
        .print()
        .await?;

    // 获取指定命名空间下的指定 Pod
    hc.do_get("/api/v1/namespaces/beta-popcloud/pods/alpine-7b74d7c9-9bzh6")
        .await?
        .print()
        .await?;

    // 获取指定命名空间下的指定 Pod 的日志
    hc.do_get("/api/v1/namespaces/beta-popcloud/pods/popmart-nginx-7bb6cc5487-qz87f/logs")
        .await?
        .print()
        .await?;

    // 在指定命名空间下的指定 Pod 中执行命令
    let pod_exec_params: serde_json::Value = json!({
        "container":Some("alpine"),
        "commands": ["/bin/sh","-c","ls -la"]
    });
    hc.do_post(
        "/api/v1/namespaces/beta-popcloud/pods/alpine-7b74d7c9-9bzh6/exec",
        pod_exec_params.clone(),
    )
    .await?
    .print()
    .await?;

    // 创建一个 Deployment
    let post_params: serde_json::Value = serde_json::json!({
        "releaseName": "popmart-nginx",
        "serviceName": "nginx",
        "containerImage": "nginx",
        "containerTag": "latest",
        "replicas": 1,
        "pullPolicy": "IfNotPresent",
        "containerPort": 80,
    });
    hc.do_post(
        "/api/v1/namespaces/beta-popcloud/deployments/nginx",
        post_params,
    )
    .await?
    .print()
    .await?;

    // 更新一个 Deployment
    let current_time = Local::now();
    let patch_params: serde_json::Value = serde_json::json!({
        "spec": {
            "template": {
                "metadata": {
                    "annotations": {
                        "redeploy-timestamp": current_time,
                    }
                },
            }
        }
    });
    hc.do_patch(
        "/api/v1/namespaces/beta-popcloud/deployments/alpine1",
        patch_params,
    )
    .await?
    .print()
    .await?;

    // 获取指定命名空间下的指定 Deployment
    hc.do_get("/api/v1/namespaces/beta-popcloud/deployments/alpine")
        .await?
        .print()
        .await?;
    Ok(())
}