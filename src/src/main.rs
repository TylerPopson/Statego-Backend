/////////////////////////////////////////////
/// main.rs
/// 
/// handles loading environment variables
/// handles setting up database connection
/// handles staring the server
/////////////////////////////////////////////

use actix_web::{web, App, HttpServer};
use mysql::{Pool, SslOpts};
use std::{env, io};

// modules from other files in project
mod models;
mod persistence;
mod queries;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // create shared data for app to be passed to functions for database connections
    let pool = set_up_environment();
    let shared_data = web::Data::new(pool);

    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        // serve functions at defined endpoints and bind global data pool
        App::new()
            .app_data(shared_data.clone())
            .service(routes::create_user)
            .service(routes::get_users)
            .service(routes::hello)
            .service(routes::login)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

// set up database connection from .env file and return a connection pool
fn set_up_environment() -> Pool {
    // import environment variables from .env file
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("setting up app from environment");

    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let db_port = db_port.parse().unwrap();

    // create a connection builder from environment
    let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    log::info!("initializing database connection");
    mysql::Pool::new(builder).unwrap()
}

// create a mysql connection builder from environment
fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password))
        .ssl_opts(SslOpts::with_danger_accept_invalid_certs(
            SslOpts::default(),
            true,
        ))
}
