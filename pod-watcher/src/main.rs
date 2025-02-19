use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    runtime::{watcher, WatchStreamExt},
    Api, Client, ResourceExt,
};
use tracing::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;
    let pod_api: Api<Pod> = Api::all(client);

    watcher(pod_api, watcher::Config::default())
        .default_backoff()
        .applied_objects()
        .for_each(|p| async move {
            match p {
                Ok(pod) => info!("{}/{}", pod.namespace().unwrap(), pod.name_any()),
                Err(e) => warn!("watch failed: {}", e),
            }
        })
        .await;

    Ok(())
}
