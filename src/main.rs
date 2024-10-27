use axum::routing::get;
use axum::Router;
use std::env;
use std::net::Ipv4Addr;
use tower_http::services::ServeDir;

mod home;
mod posts;
mod templates;

#[tokio::main]
async fn main() {
    http_server().await;
}

const PORT_KEY: &str = "FUNCTIONS_CUSTOMHANDLER_PORT";

async fn http_server() {
    let port: u16 = match env::var(PORT_KEY) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let router = Router::new()
        .route("/", get(home::home_route))
        .route("/posts", get(posts::posts_route))
        .route("/posts/:post", get(posts::post_route))
        .nest_service("/assets", ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
