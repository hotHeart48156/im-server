use crate::{
    chat_session::UserSession,
    util::{check_token_expired::check_user_token_is_expired, get_token::get_token},
};
use actix_http::Error;
use actix_web::{HttpRequest, web,HttpResponse};
use serde::{Deserialize, Serialize};
use actix_web_actors::ws;

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: String,
}

pub async fn web_stock_chat_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(id) => ws::start(UserSession { user_id: id }, &req, stream),
                None => Ok(HttpResponse::NotFound().body("user id is expired")),
            }
        }
        None => Ok(HttpResponse::NotFound().body("cannot find token in head")),
    }
}
