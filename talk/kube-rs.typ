#let orange = rgb("#D34516")
#let blue = rgb("#1E2650")

#set document(
  author: "Yannik Dällenbach",
  title: "Extend Kubernetes Using Rust: A Quick Introduction to kube-rs",
)

#set page(
  width: 160mm,
  height: 90mm,
  number-align: bottom+right,
  numbering: (..numbers) => {
    let total = numbers.pos().at(1) - 1
    let num = numbers.pos().at(0)
    if num != 1 {
      text(size: 0.8em, numbering("1/1", num - 1, total))
    }
  },
)

#set text(font: "Helvetica", size: 14pt)
#show figure.caption: set text(size: 0.4em)

#show heading.where(level: 1): x => { 
  pagebreak(weak: true)
  x.body
}

#show raw.where(block: true): it => {
  show raw.line: l => {
    text(fill: gray)[#l.number]
    h(1em)
    l.body
  }
  set text(0.6em)
  it
}

#let title = (
  title: str, 
  subtitle: str,
  author: str,
  location: str
) => {
  set page(
    background: align(top, line(stroke: 1em+orange, length: 100%))
  )
  place(horizon+left)[
    #text(size: 1.8em, weight: "bold", title)
    #linebreak()
    #text(size: 1.5em, weight: "bold", subtitle)
  ]
  place(bottom)[
    #author
    #linebreak()
    #location
  ]
}

#title(
  title: "Extend Kubernetes Using Rust:",
  subtitle: [A _Quick_ Introduction to kube-rs],
  author: "Yannik Dällenbach",
  location: "Rust Talks Luzern, 20.11.2025"
)

= Agenda

#grid(
[
- Introduction to Kubernetes
- kube-rs
- Demos
],
  image("assets/ferris+k8s.svg", width: 60%),
  columns: 2,
  align: (start, horizon+center),

)
= What Is Kubernetes?

Kubernetes is a #text(fill: orange)[portable], #text(fill: orange)[extensible], #text(fill: orange)[open source
platform for managing containerized workloads] and
services, that facilitates both declarative configuration
and automation.

#link("https://kubernetes.io/docs/concepts/overview/")

#pagebreak()

#figure(
  image("assets/kubernetes-cluster-architecture.svg"),
  caption: link("https://kubernetes.io/docs/concepts/overview/"),
  supplement: none,
)

= Kubernetes API

- Resources (`Pods`, `Deployments`, `Services`, etc.)
- Custom Resource Definitions (`CRDs`) allow extending the API
- Controllers and Operators

= Reconciliation Loop

#figure(
  image("assets/reconciliation-loop.pdf", width: 80%),
  supplement: none,
)

= Example: nginx `Deployment`

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx
spec:
  replicas: 1
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - image: nginx:latest
        name: nginx
```
= kube-rs

#grid(
  text(size: 0.8em)[
    - Rust client for Kubernetes
    - Runtime abstraction
    - Derive macro for `CRDs`
    #linebreak()
    - First release: April 29, 2019
    - CNCF Sandbox: November 16, 2022
    - Release 1.0: May 13, 2025
    - Current 2.0.1: Sept 12, 2025
  ],
  image("assets/kube-rs-logo.svg", width: 60%),
  columns: 2,
  align: (start, horizon+center),
)

= kube-rs: Runtime

- `watcher`: stream of events
- `reflector`: watcher + store
- `controller`: reflector + _n_ watchers + `reconciler`
- `reconciler`: \ #[
#show raw: set text(0.8em)
```rust async fn reconcile(o: Arc<K>, ctx: Arc<T>) -> Result<Action, Error>```
]

= Demos

1. *CRUD Operations*: Creating and managing `Deployments`
2. *Pod Watcher*: Streaming `Pod` related events in real-time
3. *Pod Reflector*: Maintaining local state cache
4. *Static Website Hosting*: Our first SaaS

= Resources

#let links = (
  "https://kube.rs",
  "https://github.com/kube-rs/version-rs",
  "https://github.com/kube-rs/controller-rs",
  "https://github.com/kube-rs/kube/issues/584",
  "https://github.com/olix0r/kubert",
  "https://ahmet.im/blog/controller-pitfalls/",
  "https://github.com/linkerd/linkerd2/tree/582f1afa5ebf0f3d227a0260bc611e987fcd261e/policy-controller",
  "https://youtu.be/rXS-3hFYVjc?si=yXjWGOQQgYnkH5-D",
  "https://github.com/ioboi/kube-rs-rust-talk",
)

#grid(
  text(size: 0.6em)[
    #for url in links {
      [ - #link(url) ]
    }
  ],
  image("assets/repository-qr-code.svg", width: 60%),
  columns: 2,
  align: (start, horizon+center),
)
