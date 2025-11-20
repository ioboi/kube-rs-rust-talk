use std::{collections::BTreeMap, sync::Arc, time::Duration};

use futures::StreamExt;
use k8s_openapi::api::{
    apps::v1::Deployment,
    core::v1::{ConfigMap, Pod, Service},
};
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{controller::Action, watcher::Config, Controller},
    Api, Client, CustomResource, Resource, ResourceExt,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Clone, Error)]
enum Error {}

#[derive(Clone)]
struct Context {
    client: Client,
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "daellenbach.org",
    version = "v1",
    kind = "Website",
    doc = "Custom resource representing a Website",
    namespaced
)]
#[kube(shortname = "web")]
pub struct WebsiteSpec {
    /// The content of a website
    pub content: String,
}

impl Website {
    async fn reconcile(&self, ctx: Arc<Context>) -> Result<Action, Error> {
        let client = ctx.client.clone();

        let name = self.name_any();
        let ns = self.namespace().unwrap();
        let content = self.spec.content.clone();

        let cm_api: Api<ConfigMap> = Api::namespaced(client.clone(), &ns);

        let oref = self.controller_owner_ref(&()).unwrap();

        let cm = ConfigMap {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                owner_references: Some(vec![oref.clone()]),
                ..Default::default()
            },
            data: Some(BTreeMap::from([(
                "index.html".to_string(),
                content.clone(),
            )])),
            ..Default::default()
        };

        cm_api
            .patch(
                name.as_str(),
                &PatchParams::apply("websitectrl"),
                &Patch::Apply(cm),
            )
            .await
            .expect("unable to create cm");

        let sha256_sum = Sha256::digest(content);

        let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), &ns);

        let mut deployment: Deployment = serde_json::from_value(json!({
            "kind": "Deployment",
            "apiVersion": "apps/v1",
            "metadata": {
                "name": name,
                "labels": {
                    "app": name
                }
            },
            "spec": {
                "replicas": 3,
                "selector": {
                    "matchLabels": {
                        "app": name
                    }
                },
                "template": {
                    "metadata": {
                        "labels": {
                            "app": name
                        },
                        "annotations": {
                            "checksum/index": format!("{:X}", sha256_sum)
                        }
                    },
                    "spec": {
                        "containers": [
                            {
                                "name": "nginx",
                                "image": "nginx:1.29",
                                "ports": [ { "containerPort": 80 } ],
                                "volumeMounts": [
                                    {
                                        "name": "nginx-index-volume",
                                        "mountPath": "/usr/share/nginx/html/index.html",
                                        "subPath": "index.html",
                                    }
                                ]
                            }
                        ],
                        "volumes": [
                            {
                                "name": "nginx-index-volume",
                                "configMap": {
                                    "name": name
                                },
                            }
                        ],
                    }
                },

            },
        }))
        .unwrap();

        deployment.metadata.owner_references = Some(vec![oref.clone()]);

        deployment_api
            .patch(
                name.as_str(),
                &PatchParams::apply("websitectrl"),
                &Patch::Apply(deployment),
            )
            .await
            .expect("unable to create deployment");

        let mut svc: Service = serde_json::from_value(json!({
            "apiVersion": "v1",
            "kind": "Service",
            "metadata": {
              "name": name
            },
            "spec": {
              "selector": {
                "app": name
              },
              "ports": [
                {
                  "protocol": "TCP",
                  "port": 80,
                  "targetPort": 80
                }
              ],
              "type": "ClusterIP"
            }
        }))
        .unwrap();

        svc.metadata.owner_references = Some(vec![oref.clone()]);

        let svc_api: Api<Service> = Api::namespaced(client, &ns);
        svc_api
            .patch(
                name.as_str(),
                &PatchParams::apply("websitectrl"),
                &Patch::Apply(svc),
            )
            .await
            .expect("unable to create service");

        Ok(Action::requeue(Duration::from_secs(5 * 60)))
    }
}

fn error_policy(_website: Arc<Website>, error: &Error, _ctx: Arc<Context>) -> Action {
    warn!("reconcile failed: {:?}", error);
    Action::requeue(Duration::from_secs(60))
}

async fn reconcile(website: Arc<Website>, ctx: Arc<Context>) -> Result<Action, Error> {
    info!("reconcile: {}", website.name_any());
    website.reconcile(ctx).await
}

pub async fn run() {
    let client = Client::try_default()
        .await
        .expect("failed to create client");

    let website_api: Api<Website> = Api::all(client.clone());
    let deployment_api: Api<Deployment> = Api::all(client.clone());
    let pod_api: Api<Pod> = Api::all(client.clone());
    let svc_api: Api<Service> = Api::all(client.clone());

    Controller::new(website_api, Config::default().any_semantic())
        .owns(deployment_api, Default::default())
        .owns(pod_api, Default::default())
        .owns(svc_api, Default::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(Context { client }))
        .for_each(|_| futures::future::ready(()))
        .await;
}
