use actix_web::{HttpRequest,HttpResponse,Error,web};
use actix_web_actors::ws;

use crate::chat_session::UserSession;
pub async fn web_stock_chat_route(req:HttpRequest,stream:web::Payload)->Result<HttpResponse,Error>{
ws::start(UserSession::default(), &req, stream)
    
}