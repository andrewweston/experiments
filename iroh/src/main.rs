mod handler;
mod node;

use std::sync::Arc;

use axum::routing::post;
use axum::Router;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let node = node::start().await.expect("should start node");
    let state = Arc::new(node);

    let router = Router::new().route("/join_doc", post(handler::join_doc)).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.expect("should bind");
    axum::serve(listener, router).await.expect("server should run");
}
