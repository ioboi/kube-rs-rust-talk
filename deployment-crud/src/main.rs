#![allow(unused_imports)]
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container, PodSpec, PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::{
    api::{DeleteParams, ListParams, ObjectMeta, Patch, PatchParams, PostParams},
    Api, Client, ResourceExt,
};
use serde_json::json;
use std::collections::BTreeMap;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;

    let deployments: Api<Deployment> = Api::default_namespaced(client);

    // let labels = BTreeMap::from([("app".to_string(), "nginx".to_string())]);
    // let deployment = Deployment {
    //     metadata: ObjectMeta {
    //         name: Some("nginx".to_string()),
    //         ..Default::default()
    //     },
    //     spec: Some(DeploymentSpec {
    //         replicas: Some(1),
    //         selector: LabelSelector {
    //             match_labels: Some(labels.clone()),
    //             ..Default::default()
    //         },
    //         template: PodTemplateSpec {
    //             metadata: Some(ObjectMeta {
    //                 labels: Some(labels.clone()),
    //                 ..Default::default()
    //             }),
    //             spec: Some(PodSpec {
    //                 containers: vec![{
    //                     Container {
    //                         name: "nginx".to_string(),
    //                         image: Some("nginx:1.29".to_string()),
    //                         ..Default::default()
    //                     }
    //                 }],
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // };

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
                            "image": "nginx:1.29",
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

    for deployment in deployments.list(&ListParams::default()).await? {
        info!("List: {:?}", deployment.name_any())
    }

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

    // deployments
    //     .delete("nginx", &DeleteParams::default())
    //     .await?;

    Ok(())
}
