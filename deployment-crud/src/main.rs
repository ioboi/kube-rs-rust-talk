#![allow(unused_imports)]
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{DeleteParams, ListParams, Patch, PatchParams, PostParams},
    Api, Client, ResourceExt,
};
use serde_json::json;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;

    let deployments: Api<Deployment> = Api::default_namespaced(client);

    let deployment: Deployment = serde_json::from_value(json!({
        "kind": "Deployment",
        "apiVersion": "apps/v1",
        "metadata": {
            "name": "nginx",
            "labels": {
                "app": "nginx"
            }
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": {
                    "app": "nginx"
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": "nginx"
                    }
                },
                "spec": {
                    "containers": [
                        {
                            "name": "nginx",
                            "image": "nginx:1.26",
                        }
                    ]
                }
            },
        },
    }))?;

    match deployments
        .create(&PostParams::default(), &deployment)
        .await
    {
        Ok(o) => info!("Created {}", o.name_any()),
        Err(e) => error!("Unable to create deployment: {:?}", e),
    }

    // for deployment in deployments.list(&ListParams::default()).await? {
    //     info!("List: {:?}", deployment.name_any())
    // }
    //
    // let deployment = deployments.get("nginx").await?;
    // info!("Get: {:?}", deployment.name_any());
    //
    // let patch = json!(
    // {
    //     "spec": {
    //         "replicas": 3
    //
    //     }
    // });
    //
    // match deployments
    //     .patch("nginx", &PatchParams::default(), &Patch::Merge(&patch))
    //     .await
    // {
    //     Ok(_) => info!("Patch successful"),
    //     Err(e) => error!("Unable to patch deployment: {:?}", e),
    // }

    deployments
        .delete("nginx", &DeleteParams::default())
        .await?;

    Ok(())
}
