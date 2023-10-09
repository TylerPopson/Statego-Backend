/////////////////////////////////////////////
/// persistence.rs
/// 
/// handles error checking on requests
/// handles authentication of users
/////////////////////////////////////////////

use actix_web::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use derive_more::{Display, Error, From};

use crate::models::{UserData, UserResponseData};

use crate::queries::{select_password_by_username, insert_new_ueser, select_user_by_id, select_all_users};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyEmail,
    EmptyUsername,
    EmptyPassword,
    BcryptError(bcrypt::BcryptError),
    MysqlError(mysql::Error),
    UnknownUser,
    Unknown,
}

//matches a PersistenceError to a StatusCode
impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyEmail => StatusCode::BAD_REQUEST,
            PersistenceError::EmptyUsername => StatusCode::BAD_REQUEST,
            PersistenceError::UnknownUser => StatusCode::UNAUTHORIZED,
            PersistenceError::EmptyPassword => StatusCode::BAD_REQUEST,
            PersistenceError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PersistenceError::MysqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PersistenceError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn create_user_verify(
    pool: &mysql::Pool,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
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
    let hashed_password = hash(password, DEFAULT_COST)?;

    let last_insert_id = insert_new_ueser(
        &mut conn,
        email,
        username,
        hashed_password,
        first_name,
        last_name,
    )?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn login_user_verify(
    pool: &mysql::Pool,
    username: String,
    password: String,
) -> Result<UserData, PersistenceError> {
    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }

    if password.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPassword);
    }

    let mut conn = pool.get_conn()?;
    let hashed_password = select_password_by_username(&mut conn, username.clone())?;

    if verify(password, &hashed_password)? {
        Ok(select_user_by_id(&mut conn, username)?)
    } else {
        Err(PersistenceError::UnknownUser)
    }
}

pub fn get_users_verify(pool: &mysql::Pool) -> Result<UserResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(UserResponseData {
        user_data: select_all_users(&mut conn)?,
    })
}
