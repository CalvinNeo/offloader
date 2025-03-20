use kube::{Client, api::{Api, PostParams}};
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::batch::v1::Job;
use serde_json::json;
use kube::ResourceExt;

const POD_NAME: &str = "calvin-offloader-pod";
const CONTAINER_NAME: &str = "calvin-offloader-container";
const IMAGE_NAME: &str = "calvin-remote-1:latest";

async fn create_pod() -> Result<(), Box<dyn std::error::Error>> {
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

async fn create_job() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    
    let jobs_api: Api<Job> = Api::default_namespaced(client);
    
    let job_manifest = json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": "calvin-remote-job",
        },
        "spec": {
            "template": {
                "spec": {
                    "containers": [{
                        "name": CONTAINER_NAME,
                        "image": IMAGE_NAME,
                        "imagePullPolicy": "Never",
                    }],
                    "restartPolicy": "Never"
                }
            },
            "backoffLimit": 4
        }
    });

    let job: Job = serde_json::from_value(job_manifest)?;

    let pp = PostParams::default();
    let created_job = jobs_api.create(&pp, &job).await?;

    println!("Created job: {:?}", created_job.metadata.name);

    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        create_job().await.unwrap();
    });
}