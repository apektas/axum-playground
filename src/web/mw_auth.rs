use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::{Error, Result};
use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;
    Ok(next.run(req).await)
}

// region - Ctx Extractor - header & url param extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("--> {:<12} - Ctx", "Extractor");

        // user cookies extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();

        // Same code as above
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO: Token components validation (e.g. signature check etc. not part of this tutorial)

        Ok(Ctx::new(user_id))

    }
}

// endregion





/// Parse a token of format user-[user-id].[expiration].[signature]
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, expiration, signature) = regex_captures!(
    r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id = user_id
        .parse::<u64>()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}


