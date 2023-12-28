use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::{Error, Result};
use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>, // not used for now (e.g. db connection)
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    // Compute Result <Ctx>
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| {c.value().to_string()});

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token){
        Ok((user_id, _exp, _signature)) => {
            //  TODO: Token component validations
            // expensive validation process
            Ok(Ctx::new(user_id))
        }
        Err(err) => Err(err)
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx_result in the request extension
    // Has to be unique by type - otherwise last write wins
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}
// region - Ctx Extractor - header & url param extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("--> {:<12} - Ctx", "Extractor");

        let d = parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExtension)?
            .clone();

        d

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


