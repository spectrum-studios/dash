use axum::extract::Request;
use axum::http::StatusCode;
use axum::routing::{ get, post };
use axum::{ Json, Router, middleware };
use bcrypt::verify;
use dash_types::auth::{ AuthErrorType, AuthToken };
use dash_types::user::{ LoginUser, RegisterUser, UserInfo };
use email_address::EmailAddress;
use http::header::AUTHORIZATION;
use http::{ HeaderMap, HeaderValue };

use crate::middleware::auth_token::auth_token;
use crate::strategies::auth_strategy::{ AuthClaims, AuthError, AuthRequestClaims, JWTClaims };
use crate::strategies::user_strategy::{ get_db_user_by_username_or_email, insert_db_user };

async fn test_auth_route(request: Request) -> Result<(StatusCode, String), AuthError> {
    let claims = AuthClaims::from_header(request.headers());
    println!("Authentication claims: {:?}", claims);
    Ok((StatusCode::OK, "Authenticated".to_string()))
}

async fn request_with_token(request: Request) -> Result<(StatusCode, HeaderMap), AuthError> {
    let claims = AuthRequestClaims::from_header(request.headers());
    if let Ok(auth_claims) = AuthClaims::new(claims.sub.clone()).await {
        let token_result = auth_claims.generate_token();
        let auth_token: AuthToken;
        match token_result {
            Ok(token) => {
                auth_token = token;
            }
            Err(error) => {
                println!("Error generating token for UUID {}: {:?}", claims.sub, error);
                return Err(error);
            }
        }

        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());
        Ok((StatusCode::CREATED, header_map.clone()))
    } else {
        Err(AuthError::from_error_type(AuthErrorType::AccessDenied))
    }
}

async fn register_user(Json(payload): Json<RegisterUser>) -> Result<
    (StatusCode, HeaderMap, Json<UserInfo>),
    AuthError
> {
    if
        payload.username.is_empty() ||
        payload.email.email().is_empty() ||
        payload.password.is_empty()
    {
        return Err(AuthError::from_error_type(AuthErrorType::MissingFields));
    }

    if !EmailAddress::is_valid(&payload.email.email()) {
        return Err(AuthError::from_error_type(AuthErrorType::InvalidEmail));
    }

    let result = insert_db_user(payload).await;
    if let Err(error) = result {
        println!("Error creating user: {}", error);
        if error.to_string().contains("duplicate key") {
            return Err(AuthError::from_error_type(AuthErrorType::UserExists));
        } else {
            return Err(AuthError::from_error_type(AuthErrorType::ServerError));
        }
    }

    let user = result.unwrap();
    let user_info = UserInfo::from_user(user);
    let token_result = AuthRequestClaims::new(user_info.uuid.clone()).await
        .unwrap()
        .generate_token();
    let auth_token: AuthToken;
    match token_result {
        Ok(token) => {
            auth_token = token;
        }
        Err(error) => {
            println!("Error generating token for UUID {}: {:?}", user_info.uuid, error);
            return Err(error);
        }
    }

    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());
    Ok((StatusCode::CREATED, header_map.clone(), Json(user_info)))
}

async fn login_user(Json(payload): Json<LoginUser>) -> Result<
    (StatusCode, HeaderMap, Json<UserInfo>),
    AuthError
> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::from_error_type(AuthErrorType::WrongCredentials));
    }

    let result = get_db_user_by_username_or_email(payload.username).await;
    if let Err(_) = result {
        return Err(AuthError::from_error_type(AuthErrorType::UserNotExist));
    }

    let user = result.unwrap();
    if verify(payload.password, &user.password).unwrap() {
        let user_info = UserInfo::from_user(user);
        let token_result = AuthRequestClaims::new(user_info.uuid.clone()).await
            .unwrap()
            .generate_token();
        let auth_token: AuthToken;
        match token_result {
            Ok(token) => {
                auth_token = token;
            }
            Err(error) => {
                println!("Error generating token for UUID {}: {:?}", user_info.uuid, error);
                return Err(error);
            }
        }

        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(&auth_token.to_string()).unwrap());
        Ok((StatusCode::OK, header_map.clone(), Json(user_info)))
    } else {
        Err(AuthError::from_error_type(AuthErrorType::WrongCredentials))
    }
}

pub fn routes() -> Router {
  Router::new()
      .merge(
          Router::new()
              .route("/test", get(test_auth_route))
              .layer(middleware::from_fn(auth_token::<AuthClaims>))
      )
      .merge(
          Router::new()
              .route("/request", get(request_with_token))
              .layer(middleware::from_fn(auth_token::<AuthRequestClaims>))
      )
      .route("/register", post(register_user))
      .route("/login", post(login_user))
}
