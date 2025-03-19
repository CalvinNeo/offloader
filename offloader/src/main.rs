use kube::{Client, api::{Api, PostParams}};
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use kube::ResourceExt;

const POD_NAME: &str = "calvin-offloader-pod";
const CONTAINER_NAME: &str = "calvin-offloader-container";
const IMAGE_NAME: &str = "calvin-remote-1:latest";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    
    let pods: Api<Pod> = Api::default_namespaced(client);
    
    let pod_spec = json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": CONTAINER_NAME
        },
        "spec": {
            "containers": [{
                "name": CONTAINER_NAME,
                "image": IMAGE_NAME,
                "imagePullPolicy": "Never",
            }]
        }
    });
    
    let pod = serde_json::from_value(pod_spec)?;
    let pod = pods.create(&PostParams::default(), &pod).await?;
    println!("Pod created: {}", pod.metadata.name.unwrap());
    
    Ok(())
}