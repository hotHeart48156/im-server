pub mod user;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: String,
}

use crate::{chat_session::UserSession, util::{check_token_expired::check_user_token_is_expired, get_token::get_token}};
pub async fn web_stock_chat_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("web_stock_chat_route");
    let token=get_token(&req);
    match token {
        Some(tk) => {
            println!("tk--{}",std::str::from_utf8(tk.as_bytes()).unwrap());
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap()) {
                Some(id) => {println!("tokem-id--{}",id); ws::start(UserSession{user_id:id}, &req, stream)},
                None => {Ok(HttpResponse::NotFound().body("user id is expired"))},
            }       },
        None => {Ok(HttpResponse::NotFound().body("cannot find token in head"))},
    }

    
}

pub fn scoped_function(cfg: &mut web::ServiceConfig) {
    cfg
    .service(user::login::login)
    .service(user::register::register)
    .service(user::friend::add_friend)
    .service(user::friend::delete_friend)
    .service(user::friend::list_friend)
    .service(user::room::add_room)
    .service(user::room::delete_room)
    .service(user::room::list_room)

    ;
}
