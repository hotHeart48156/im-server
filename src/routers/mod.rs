pub mod user;

use actix_session::Session;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: String,
}

use crate::chat_session::UserSession;
pub async fn web_stock_chat_route(
    req: HttpRequest,
    session: Session,
    stream: web::Payload,
    id: web::Query<Id>,
) -> Result<HttpResponse, Error> {
    let res:Option<String>=session.get(id.id.clone().as_str()).unwrap();
    match res {
        Some(_) => {ws::start(UserSession{user_id:id.id.to_owned()}, &req, stream)},
        None => {Ok(HttpResponse::NotFound().body("user id expired"))},
    }
    
}
