use std::env;

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::response::{ IntoResponse, Response };
use axum::{ Json, RequestPartsExt };
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use base64::prelude::*;
use dash_types::auth::{ AuthErrorBody, AuthErrorType, AuthToken };
use http::request::Parts;
use http::{ HeaderMap, StatusCode };
use jsonwebtoken::{
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
    decode,
    encode,
    get_current_timestamp,
};
use once_cell::sync::Lazy;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use struct_iterable::Iterable;

use super::user_strategy::get_db_user_by_uuid;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("Missing JWT_SECRET environment variable");
    Keys::new(secret.as_bytes())
});

static JWT_AUDIENCE: Lazy<String> = Lazy::new(||
    env::var("JWT_AUDIENCE").expect("Missing JWT_AUDIENCE environment variable")
);

static JWT_ISSUER: Lazy<String> = Lazy::new(||
    env::var("JWT_ISSUER").expect("Missing JWT_ISSUER environment variable")
);

static AUTH_TOKEN_EXPIRY: Lazy<u64> = Lazy::new(|| {
    let expiry = env
        ::var("AUTH_TOKEN_EXPIRY")
        .expect("Missing AUTH_TOKEN_EXPIRY environment variable");
    u64::from_str_radix(&expiry, 10).expect("Cannot parse AUTH_TOKEN_EXPIRY as u64")
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub trait JWTClaims {
    async fn new(uuid: String) -> Result<Self, AuthError> where Self: Sized;

    fn from_header(header: &HeaderMap) -> Self where Self: for<'de> Deserialize<'de> {
        let claims = header.get("X-Claims").unwrap();
        serde_json
            ::from_str(&String::from_utf8(BASE64_STANDARD.decode(claims).unwrap()).unwrap())
            .unwrap()
    }

    fn from_string(encoded_str: &str) -> Result<Self, AuthError>
        where Self: Sized, Self: for<'de> Deserialize<'de>
    {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 5;
        validation.set_audience(&[JWT_AUDIENCE.clone()]);
        validation.set_issuer(&[JWT_ISSUER.clone()]);

        match decode::<Self>(encoded_str, &KEYS.decoding, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(AuthError::from_error_type(AuthErrorType::InvalidToken)),
        }
    }

    fn generate_token(&self) -> Result<AuthToken, AuthError> where Self: Serialize {
        match encode(&Header::default(), &self, &KEYS.encoding) {
            Ok(encoded_string) => Ok(AuthToken::new(encoded_string)),
            Err(error) => {
                println!("Error generating token: {:?}", error);
                Err(AuthError::from_error_type(AuthErrorType::TokenGeneration))
            }
        }
    }
}

async fn from_request_parts<T>(parts: &mut Parts) -> Result<T, AuthError>
    where T: for<'de> Deserialize<'de>
{
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>().await
        .map_err(|_| AuthError::from_error_type(AuthErrorType::InvalidToken))?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 5;
    validation.set_audience(&[JWT_AUDIENCE.clone()]);
    validation.set_issuer(&[JWT_ISSUER.clone()]);

    let token_data = decode::<T>(bearer.token(), &KEYS.decoding, &validation).map_err(|_|
        AuthError::from_error_type(AuthErrorType::InvalidToken)
    )?;
    Ok(token_data.claims)
}

#[derive(Debug, Deserialize, Iterable, Serialize)]
pub struct AuthClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub acc: bool,
    pub iat: usize,
}

impl JWTClaims for AuthClaims {
    async fn new(uuid: String) -> Result<Self, AuthError> {
        match get_db_user_by_uuid(uuid).await {
            Ok(user) =>
                Ok(Self {
                    iss: JWT_ISSUER.clone(),
                    sub: user.uuid,
                    aud: JWT_AUDIENCE.clone(),
                    exp: get_current_timestamp() + *AUTH_TOKEN_EXPIRY,
                    acc: user.is_admin,
                    iat: get_current_timestamp() as usize,
                }),
            Err(_) => Err(AuthError::from_error_type(AuthErrorType::TokenGeneration)),
        }
    }
}

impl<S> FromRequestParts<S> for AuthClaims where S: Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        from_request_parts::<AuthClaims>(parts).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequestClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub iat: usize,
}

impl JWTClaims for AuthRequestClaims {
    async fn new(uuid: String) -> Result<Self, AuthError> {
        Ok(Self {
            iss: JWT_ISSUER.clone(),
            sub: uuid,
            aud: JWT_AUDIENCE.clone(),
            exp: get_current_timestamp() + *AUTH_TOKEN_EXPIRY,
            iat: get_current_timestamp() as usize,
        })
    }
}

impl<S> FromRequestParts<S> for AuthRequestClaims where S: Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        from_request_parts::<AuthRequestClaims>(parts).await
    }
}

#[derive(Debug)]
pub struct AuthError(dash_types::auth::AuthError);

impl AuthError {
    pub fn from_error_type(error_type: AuthErrorType) -> Self {
        Self { 0: dash_types::auth::AuthError::from_error_type(error_type) }
    }

    pub fn status(&self) -> StatusCode {
        self.0.status.to_owned()
    }

    pub fn body(&self) -> AuthErrorBody {
        self.0.body.to_owned()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        (self.status(), Json(json!(self.body()))).into_response()
    }
}
