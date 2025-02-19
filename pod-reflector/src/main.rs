use axum::{extract::State, routing, Json, Router};
use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    runtime::{reflector, watcher, WatchStreamExt},
    Api, Client, ResourceExt,
};

type Cache = reflector::Store<Pod>;

async fn get_pods(State(store): State<Cache>) -> Json<Vec<String>> {
    let pods = store
        .state()
        .iter()
        .map(|pod| format!("{}/{}", pod.namespace().unwrap(), pod.name_any()))
        .collect();
    Json(pods)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;
    let pod_api: Api<Pod> = Api::all(client);

    let (reader, writer) = reflector::store();
    let stream = reflector(writer, watcher(pod_api, Default::default()))
        .touched_objects()
        .for_each(|_| futures::future::ready(()));

    tokio::spawn(stream);

    let app = Router::new()
        .route("/", routing::get(get_pods))
        .with_state(reader);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
