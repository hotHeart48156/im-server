use actix_web::{App, HttpServer,web};

use crate::routers::web_stock_chat_route;

pub  async fn  start_server(){
let addr=dotenv::var("SERVER_URL").unwrap();

   let server=HttpServer::new(
        ||{
            App::new()
            .service(web::resource("/ws").to(web_stock_chat_route))
        }
    ).bind(&addr.as_str()).unwrap();
    server.run().await.unwrap();
}