use axum::extract::{ Json, Request };
use axum::http::StatusCode;
use axum::routing::{ delete, get };
use axum::{ RequestExt, Router, middleware };
use dash_types::auth::AuthErrorType;
use dash_types::user::UserInfo;

use crate::middleware::auth_token::auth_token;
use crate::strategies::auth_strategy::{ AuthClaims, AuthError, AuthRequestClaims, JWTClaims };
use crate::strategies::user_strategy::{ delete_user_by_uuid, get_all_users, get_db_user_by_uuid };

async fn get_user_info(request: Request) -> Result<(StatusCode, Json<UserInfo>), AuthError> {
    let claims = AuthRequestClaims::from_header(request.headers());
    match get_db_user_by_uuid(claims.sub).await {
        Ok(user) => Ok((StatusCode::OK, axum::Json(UserInfo::from_user(user)))),
        Err(_) => Err(AuthError::from_error_type(AuthErrorType::UserNotExist)),
    }
}

async fn get_all_user_info(
    request: Request
) -> Result<(StatusCode, Json<Vec<UserInfo>>), AuthError> {
    let claims = AuthClaims::from_header(request.headers());
    if claims.acc {
        match get_all_users().await {
            Ok(users) => Ok((StatusCode::OK, axum::Json(users))),
            Err(_) => Err(AuthError::from_error_type(AuthErrorType::UserNotExist)),
        }
    } else {
        Err(AuthError::from_error_type(AuthErrorType::AccessDenied))
    }
}

async fn delete_user(request: Request) -> Result<StatusCode, AuthError> {
    let claims = AuthClaims::from_header(request.headers());
    let uuid = request.extract().await;
    match uuid {
        Ok(uuid) => {
            if claims.acc {
                match delete_user_by_uuid(uuid).await {
                    Ok(_) => Ok(StatusCode::OK),
                    Err(_) => Err(AuthError::from_error_type(AuthErrorType::UserNotExist)),
                }
            } else {
                Err(AuthError::from_error_type(AuthErrorType::AccessDenied))
            }
        }
        Err(error) => {
            println!("{}", error);
            Err(AuthError::from_error_type(AuthErrorType::ServerError))
        }
    }
}

pub fn routes() -> Router {
  Router::new()
      .route(
          "/info",
          get(get_user_info).layer(middleware::from_fn(auth_token::<AuthRequestClaims>))
      )
      .route("/all", get(get_all_user_info).layer(middleware::from_fn(auth_token::<AuthClaims>)))
      .route("/", delete(delete_user).layer(middleware::from_fn(auth_token::<AuthClaims>)))
}
