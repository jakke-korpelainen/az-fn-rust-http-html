# Azure Function (HTTP) with Rust serving HTML

Simple barebones blog that pulls data from contentful space. Might add a /frontend into this to demonstrate mixing rust and js in the same project.

## Built with

Azure Functions, Rust, Contentful

**Rust dependencies**

axum, askama, contentful, tokio, reqwest, serde, tower_http

## Prequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Azure Functions Core Tools](https://learn.microsoft.com/en-us/azure/azure-functions/functions-run-local?tabs=windows%2Cisolated-process%2Cnode-v4%2Cpython-v2%2Chttp-trigger%2Ccontainer-apps&pivots=programming-language-csharp)

## Demo

[https://jakke-fi-function.azurewebsites.net/](https://jakke-fi-function.azurewebsites.net/)

## Instructions

### Configurations

**Envs**

```
CONTENTFUL_SPACE_ID= # Your contentful space id to pull data from
CONTENTFUL_ACCESS_TOKEN= # Your contentful access token for the space
```

**Contentful Data Model for Blogs**

The parsing expects the blog posts to be in the following format:

```rust
pub struct BlogPost {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
```

and be typed as `blogPost` in contentful. See [src\posts.rs](src\posts.rs) for more details.

### Running the cargo

1. `cargo run`
2. See [http://localhost:3000](http://localhost:3000)

### Running locally the Azure Function
1. `release.sh` builds the project and copies `handler.exe` into folder root.
2. `func start`
3. See [http://localhost:7071](http://localhost:7071)