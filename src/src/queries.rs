/////////////////////////////////////////////
/// queeries.rs
/// 
/// handles querying the mysql database
/////////////////////////////////////////////

use mysql::{params, prelude::*};

use crate::models::UserData;

pub fn insert_new_ueser(
    conn: &mut mysql::PooledConn,
    my_email: String,
    my_username: String,
    my_password: String,
    my_first_name: String,
    my_last_name: String,
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

//TODO select all fields from the user table
pub fn select_all_users(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<UserData>> {
    conn.query_map(
        "
        SELECT id, email, username, first_name, last_name
        FROM users
        ",
        |(my_id, my_email, my_username, my_first_name, my_last_name)| UserData {
            id: my_id,
            email: my_email,
            username: my_username,
            first_name: my_first_name,
            last_name: my_last_name,
        },
    )
}

pub fn select_password_by_username(conn: &mut mysql::PooledConn, username: String) -> mysql::error::Result<String> {
    conn.exec_first(
        "
        SELECT pass
        FROM users
        WHERE username = :username
        ",
        params! {
            "username" => username
        },
    )
    .map(|pass| pass.unwrap())
}

pub fn select_user_by_id(
    conn: &mut mysql::PooledConn,
    username: String,
) -> mysql::error::Result<UserData> {
    conn.exec_first(
        "
        SELECT id, email, username, first_name, last_name
        FROM users
        WHERE username = :username
        ",
        params! {
            "username" => username
        },
    )
    .map(|user| user.unwrap())
}
