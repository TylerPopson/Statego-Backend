use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*};

use crate::models::{
    UserDetails
};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyEmail,
    EmptyUsername,
    EmptyPassword,

    MysqlError(mysql::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyEmail
            | PersistenceError::EmptyUsername
            | PersistenceError::EmptyPassword => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn create_user(
    pool: &mysql::Pool,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String
) -> Result<(), PersistenceError> {
    if email.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyEmail);
    }

    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }

    if password.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPassword);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id =
        insert_user_data(&mut conn, email, username, password, first_name, last_name)?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

fn insert_user_data(
    conn: &mut mysql::PooledConn,
    my_email: String,
    my_username: String,
    my_password: String,
    my_first_name: String,
    my_last_name: String
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO users (email, username, pass, first_name, last_name)
        VALUES (:email, :username,:pass, :first_name, :last_name)
        ",
        params! {
            "email" => my_email,
            "username" => my_username,
            "pass" => my_password,
            "first_name" => my_first_name,
            "last_name" => my_last_name,
        },
    )
    .map(|_| conn.last_insert_id())
}