use im_server::server::start_server;

#[actix_web::main]

async fn main() {
    start_server().await;
}
