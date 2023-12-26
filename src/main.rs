
use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use axum::routing::{get, get_service};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod error;
mod web;



#[tokio::main]
async fn main() {

    //region: Server
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    //endregion: Server


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    // region: Model

    // endregion: Model
}

// region - Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello_path_variable))
}

// endregion


// region - Static File Serve via tower-http service
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// endregion

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

// endregion

async fn handler_hello_path_variable(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler hello - {name:?}", "HANDLER");

    Html(format!("Hello {name}!!!"))
}