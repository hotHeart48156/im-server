use crate::routers::{chat_server::web_stock_chat_route, scoped_function};
use actix_cors::Cors;
use actix_http::http;
use actix_redis::RedisSession;
use actix_web::{Result,App, HttpServer, dev, middleware::{self, errhandlers::{ErrorHandlerResponse, ErrorHandlers}}, web};
use diesel::{self, r2d2::ConnectionManager, PgConnection};
use rand::Rng;
fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
pub async fn start_server() {
    let addr = dotenv::var("SERVER_URL").unwrap();
    let redis_addr = std::env::var("REDIS_URL").unwrap();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();
    let private_key = rand::thread_rng().gen::<[u8; 32]>();


    
        
    let server = HttpServer::new(move || {
        
        let cors=Cors::default().supports_credentials()
        .allow_any_method()
        .allow_any_header()
        .allow_any_origin()
        .expose_any_header()
        ;
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .wrap(
                RedisSession::new(redis_addr.to_owned(), &private_key)
                    .cookie_name("im-server")
                    .cookie_path("/ws")
                    .cookie_http_only(false)
            )
            .wrap(
                ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
            )
            .data(pg_pool())
            .service(web::resource("/ws").to(web_stock_chat_route))
            .configure(scoped_function)
    })
    .workers(1)
    .bind(&addr.as_str())
    .unwrap();
    println!("server running {}", addr);
    // server
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
