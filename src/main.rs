
use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::{middleware, Router};
use axum::routing::{get, get_service};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::{CookieManager, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use crate::model::ModelController;


mod ctx;

mod error;
mod web;
mod model;


#[tokio::main]
async fn main() -> Result<()>{

    // Initialize the ModelController
    let mc = ModelController::new().await?;

    // only apply middle-ware for api endpoint.
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    //region: Server
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver
        ))
        // Cookie manager should first executed - ordering is matter here
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    //endregion: Server


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    // region: Model

    // endregion: Model

    Ok(())
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

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}