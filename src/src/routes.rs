use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    models::{UserDetails},
    persistence::{
        create_user,
    },
};

#[post("/user")]
pub(crate) async fn add_user(
    web::Json(user_data): web::Json<UserDetails>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {

    let email = user_data.email;
    let username = user_data.username;
    let password = user_data.pass;
    let first_name = user_data.first_name;
    let last_name = user_data.last_name;

    web::block(move || create_user(&data, email, username, password, first_name, last_name)).await??;

    Ok(HttpResponse::NoContent())
}