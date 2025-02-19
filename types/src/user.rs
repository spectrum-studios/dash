use std::fmt;
use std::str::FromStr;

use email_address::EmailAddress;
use serde::{ Deserialize, Serialize };
#[cfg(feature = "sqlx")]
use sqlx::any::AnyRow;
#[cfg(feature = "sqlx")]
use sqlx::{ FromRow, Row };

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub email: EmailAddress,
    pub password: String,
    pub is_admin: bool,
}

#[cfg(feature = "sqlx")]
impl<'r> FromRow<'r, AnyRow> for User {
    fn from_row(row: &AnyRow) -> Result<Self, sqlx::Error> {
        let id: i32 = row.try_get("id")?;
        let uuid: String = row.try_get("uuid")?;
        let username: String = row.try_get("username")?;
        let email: EmailAddress = match row.try_get::<String, &str>("email") {
            Ok(address) => EmailAddress::new_unchecked(address),
            Err(e) => {
                println!("Error: {}", e);
                EmailAddress::new_unchecked("")
            }
        };
        let password: String = row.try_get("password")?;
        let is_admin = row.try_get("is_admin")?;

        Ok(Self { id, uuid, username, email, password, is_admin })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: EmailAddress,
    pub password: String,
}

impl fmt::Display for RegisterUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Username: {}\nEmail: {}\nPassword: {}",
            self.username,
            self.email.email(),
            self.password
        )
    }
}

impl RegisterUser {
    pub fn update_field(&self, key: &str, value: String) -> Result<Self, String> {
        let mut user = self.clone();
        match key {
            "username" => {
                user.username = value;
            }
            "email" => {
                user.email = EmailAddress::from_str(&value).unwrap();
            }
            "password" => {
                user.password = value;
            }
            _ => {
                return Err(format!("Invalid key: {}", key));
            }
        }
        Ok(user)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl fmt::Display for LoginUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Username: {}\nPassword: {}", self.username, self.password)
    }
}

impl LoginUser {
    pub fn update_field(&self, key: &str, value: String) -> Result<Self, String> {
        let mut user = self.clone();
        match key {
            "username" => {
                user.username = value;
            }
            "password" => {
                user.password = value;
            }
            _ => {
                return Err(format!("Invalid key: {}", key));
            }
        }
        Ok(user)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct UserInfo {
    pub uuid: String,
    pub username: String,
    pub is_admin: bool,
}

impl fmt::Display for UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID: {}\nUsername: {}\nAdmin: {}", self.uuid, self.username, self.is_admin)
    }
}

impl UserInfo {
    pub fn new() -> Self {
        Self { uuid: String::new(), username: String::new(), is_admin: false }
    }

    pub fn from_user(user: User) -> Self {
        Self { uuid: user.uuid, username: user.username, is_admin: user.is_admin }
    }
}
