use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    models::{UserDetails},
    persistence::{
        create_user, get_user,
    },
};

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, cruel world")
}

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

#[get("/user")]
pub(crate) async fn get_users(data: web::Data<mysql::Pool>,) -> actix_web::Result<impl Responder> {
    
    let users = web::block(move || get_user(&data)).await??;
    Ok(web::Json(users))
}