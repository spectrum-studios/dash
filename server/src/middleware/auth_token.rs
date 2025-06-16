use std::fmt::Debug;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use base64::prelude::*;
use http::HeaderValue;
use serde::Serialize;
use serde_json::json;

use crate::strategies::auth_strategy::JWTClaims;

pub async fn auth_token<T>(claims: T, mut request: Request, next: Next) -> Response
where
    T: JWTClaims,
    T: Serialize,
    T: Debug,
{
    let header_map = request.headers_mut();
    while header_map.contains_key("X-Claims") {
        header_map.remove("X-Claims");
    }
    let json = json!(claims);
    let encoded_text = BASE64_STANDARD.encode(json.to_string());
    request.headers_mut().insert("X-Claims", HeaderValue::from_str(&encoded_text).unwrap());
    next.run(request).await
}
