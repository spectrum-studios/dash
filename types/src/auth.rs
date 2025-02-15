use http::StatusCode;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthToken {
    pub token: String,
    pub token_type: String,
}

impl AuthToken {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: String::from("Bearer"),
        }
    }

    pub fn default() -> Self {
        Self {
            token: String::new(),
            token_type: String::new(),
        }
    }

    pub fn from_string(string: String) -> Self {
        Self {
            token: string,
            token_type: String::from("Bearer"),
        }
    }

    pub fn to_string(self: Self) -> String {
        self.token
    }
}

#[derive(Clone, Debug)]
pub struct AuthError {
    pub status: StatusCode,
    pub body: AuthErrorBody,
}

impl AuthError {
    pub fn default() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: AuthErrorBody {
                error_type: AuthErrorType::ServerError,
                message: String::from("Default authentication error"),
            },
        }
    }

    pub fn from_error_type(error_type: AuthErrorType) -> Self {
        let (status, message) = match error_type {
            AuthErrorType::ServerError =>
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("Server error")),
        };

        Self {
            status,
            body: AuthErrorBody {
                error_type,
                message,
            },
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status.to_owned()
    }

    pub fn body(&self) -> AuthErrorBody {
        self.body.to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthErrorBody {
    pub error_type: AuthErrorType,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthErrorType {
    ServerError,
}
