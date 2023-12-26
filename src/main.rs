
use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use tokio::net::TcpListener;
mod error;



#[tokio::main]
async fn main() {

    //region: Server
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello))
        .route("/hello2/:name", get(handler_hello_path_variable));

    //endregion: Server


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();

    // region: Model

    // endregion: Model
}

#[derive(Debug, Deserialize)]
struct  HelloParams {
    name: Option<String>,
}


// region - handler Hello
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler hello", "HANDLER");
    println!("->> {:<12} - handler hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello {name}!!!"))
}


async fn handler_hello_path_variable(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler hello - {name:?}", "HANDLER");

    Html(format!("Hello {name}!!!"))
}