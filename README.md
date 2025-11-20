# Extend Kubernetes Using Rust: A Quick Introduction to kube-rs

Talk presented at:

- [**2025 Rust Talks Bern #1 @Puzzle ITC AG**](https://www.meetup.com/rust-bern/events/305597994)
- [**2025 Rust Talks Luzern #3 @Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681)

## Overview

This repository contains the presentation materials and code examples for a talk introducing [kube-rs](https://kube.rs/),
the Rust crate for interacting with Kubernetes.
The talk demonstrates how to extend Kubernetes using Rust, covering basic API interactions, watching resources, and building custom controllers.

## Contents

- **`talk/`** - Presentation slides
- **`deployment-crud/`** - Example demonstrating CRUD operations on Kubernetes Deployments
- **`pod-watcher/`** - Example showing how to watch Pod events in real-time
- **`pod-reflector/`** - Example implementing a reflector pattern for Pod state
- **`website/`** - Example controller for static website hosting

## Resources

- [https://kube.rs](https://kube.rs)
- [https://github.com/kube-rs/version-rs](https://github.com/kube-rs/version-rs)
- [https://github.com/kube-rs/controller-rs](https://github.com/kube-rs/controller-rs)
- [https://github.com/kube-rs/kube/issues/584](https://github.com/kube-rs/kube/issues/584)
- [https://github.com/olix0r/kubert](https://github.com/olix0r/kubert)
- [https://ahmet.im/blog/controller-pitfalls/](https://ahmet.im/blog/controller-pitfalls/)
- [https://github.com/linkerd/linkerd2/tree/582f1afa5ebf0f3d227a0260bc611e987fcd261e/policy-controller](https://github.com/linkerd/linkerd2/tree/582f1afa5ebf0f3d227a0260bc611e987fcd261e/policy-controller)
- [https://youtu.be/rXS-3hFYVjc?si=yXjWGOQQgYnkH5-D](https://youtu.be/rXS-3hFYVjc?si=yXjWGOQQgYnkH5-D)
