use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{Error, Result, web};


pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}


async fn api_login(cookies:Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: implement real db/auth login

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "DDuser-1.exp.sign"));
    // TODO: Set cookies

    // Create success body
    let payload = Json(
        json!( {
            "result": {
                "success":  true
            }
        })
    );

    Ok(payload)
}

#[derive(Debug, Deserialize)]
struct LoginPayload{
    username: String,
    pwd: String,
}

