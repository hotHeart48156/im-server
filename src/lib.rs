pub mod schema;
pub mod models;
pub mod db;
pub mod message;
pub mod chat_server;
pub mod chat_session;
pub mod server;
pub mod routers;
pub mod test;
pub mod util;
// pub mod linux_daemon;
extern crate windows_service;
#[macro_use]
extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
