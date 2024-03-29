
use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::{Json, middleware, Router};
use axum::http::{Method, StatusCode, Uri};
use axum::routing::{get, get_service};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::{CookieManager, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::log::log_request;
use crate::model::ModelController;


mod ctx;

mod error;
mod web;
mod model;

mod log;

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

    (StatusCode::OK, Html(format!("Hello {name}!!!")))
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");


    let uuid = Uuid::new_v4();

    // Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error =  service_error.map(|se| se.client_status_and_error());

    // if
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    //
    // --> Same as this in scala:
    // foo: Option[(String, Int)]; foo.map(_._2)
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!("->> server log line {:<12} Error: {service_error:?}", uuid);



    println!();
    error_response.unwrap_or(res)
}