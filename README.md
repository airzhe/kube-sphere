## kube-sphere

This service is a Kubernetes HTTP API invocation service, encapsulated with the kube-rs package and the Axum framework.

test

```bash
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```



Pod管理

创建Pod: POST /api/v1/namespaces/{namespace}/pods
获取所有Pod: GET /api/v1/pods
获取某个命名空间下的所有Pod: GET /api/v1/namespaces/{namespace}/pods
获取某个Pod的信息: GET /api/v1/namespaces/{namespace}/pods/{name}
删除Pod: DELETE /api/v1/namespaces/{namespace}/pods/{name}
Service管理

创建Service: POST /api/v1/namespaces/{namespace}/services
获取所有Service: GET /api/v1/services
获取某个命名空间下的所有Service: GET /api/v1/namespaces/{namespace}/services
获取某个Service的信息: GET /api/v1/namespaces/{namespace}/services/{name}
删除Service: DELETE /api/v1/namespaces/{namespace}/services/{name}
Deployment管理

创建Deployment: POST /apis/apps/v1/namespaces/{namespace}/deployments
获取所有Deployment: GET /apis/apps/v1/deployments
获取某个命名空间下的所有Deployment: GET /apis/apps/v1/namespaces/{namespace}/deployments
获取某个Deployment的信息: GET /apis/apps/v1/namespaces/{namespace}/deployments/{name}
删除Deployment: DELETE /apis/apps/v1/namespaces/{namespace}/deployments/{name}
Node管理

获取所有Node: GET /api/v1/nodes
获取某个Node的信息: GET /api/v1/nodes/{name}
Namespace管理

创建Namespace: POST /api/v1/namespaces
获取所有Namespace: GET /api/v1/namespaces
获取某个Namespace的信息: GET /api/v1/namespaces/{name}
删除Namespace: DELETE /api/v1/namespaces/{name}


cargo watch -q -c -w src/ -x "run"
cargo watch -q -c -w examples/ -x "run --example quick_dev"