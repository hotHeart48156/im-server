use crate::routers::{
    user::{login::login, register::register},
    web_stock_chat_route,
};
use actix_redis::RedisSession;
use actix_web::{middleware, web, App, HttpServer};
use diesel::{self, r2d2::ConnectionManager, PgConnection};
pub async fn start_server() {
    let addr = dotenv::var("SERVER_URL").unwrap();
    let redis_addr = std::env::var("REDIS_URL").unwrap();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(RedisSession::new(redis_addr.to_owned(), &[0; 32]))
            .data(pg_pool())
            .service(web::resource("/ws").to(web_stock_chat_route))
            .service(login)
            .service(register)
    })
    .bind(&addr.as_str())
    .unwrap();
    server.run().await.unwrap();
}

pub type DbPoolType = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn pg_pool() -> DbPoolType {
    let pg_url = std::env::var("DATABASE_URL").unwrap();
    let manage = ConnectionManager::<PgConnection>::new(pg_url);
    let pool = r2d2::Pool::builder()
        .build(manage)
        .expect("cant create database pool");
    pool
}
