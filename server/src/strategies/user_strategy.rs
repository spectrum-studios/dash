use std::convert::TryInto;
use std::env;

use bcrypt::{ DEFAULT_COST, hash_with_salt };
use dash_types::user::{ RegisterUser, User, UserInfo };
use once_cell::sync::Lazy;
use sqlx::any::AnyQueryResult;
use uuid::Uuid;

use crate::pool;

static PASSWORD_SALT: Lazy<[u8; 16]> = Lazy::new(|| {
    env::var("PASSWORD_SALT")
        .expect("Missing PASSWORD_SALT environment variable")
        .as_bytes()
        .try_into()
        .expect("PASSWORD_SALT is not 16 characters long")
});

pub async fn get_all_users() -> Result<Vec<UserInfo>, sqlx::Error> {
    let query = "SELECT * FROM \"users\";";
    sqlx::query_as::<_, UserInfo>(query).fetch_all(&pool::get_pool()).await
}

pub async fn get_db_user_by_username_or_email(
    username_or_email: String
) -> Result<User, sqlx::Error> {
    let query = "SELECT * FROM \"users\" WHERE username = $1 OR email = $1;";
    sqlx::query_as::<_, User>(query).bind(username_or_email).fetch_one(&pool::get_pool()).await
}

pub async fn get_db_user_by_uuid(uuid: String) -> Result<User, sqlx::Error> {
    let query = "SELECT * FROM \"users\" WHERE uuid = $1;";
    sqlx::query_as::<_, User>(query).bind(uuid).fetch_one(&pool::get_pool()).await
}

pub async fn delete_user_by_uuid(uuid: String) -> Result<AnyQueryResult, sqlx::Error> {
    let query = "DELETE FROM \"users\" WHERE uuid = $1;";
    sqlx::query(query).bind(uuid).execute(&pool::get_pool()).await
}

pub async fn insert_db_user(register_user: RegisterUser) -> Result<User, sqlx::Error> {
    let uuid = Uuid::new_v4();
    let mut salt: [u8; 16] = [0; 16];
    salt.copy_from_slice(&PASSWORD_SALT.clone());

    let query =
        "INSERT INTO \"users\" (uuid, username, email, password, is_admin)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;";
    sqlx
        ::query_as::<_, User>(query)
        .bind(uuid.to_string())
        .bind(register_user.username)
        .bind(register_user.email.to_string())
        .bind(hash_with_salt(register_user.password, DEFAULT_COST, salt).unwrap().to_string())
        .bind(false)
        .fetch_one(&pool::get_pool()).await
}
