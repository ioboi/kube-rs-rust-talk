use kube::CustomResourceExt;
fn main() {
    print!(
        "{}",
        serde_yaml::to_string(&website::Website::crd()).unwrap()
    )
}
