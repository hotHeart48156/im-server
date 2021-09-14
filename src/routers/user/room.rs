use actix_session::Session;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};

use crate::{
    db::room_operator::RoomsOperator,
    models::rooms_model::{PostRoom},
    server::DbPoolType,
    util::{check_token_expired::check_user_token_is_expired, get_token::get_token},
};
#[post("/add_room")]
pub async fn add_room(
    session: Session,
    pool: web::Data<DbPoolType>,
    room: web::Json<PostRoom>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    handle(&session,  req, |_| {
        let room_operator = RoomsOperator { conn: &pool };
        room_operator
            .create_room(room.user_id.parse::<i32>().unwrap())
            .unwrap();
        Ok(HttpResponse::Ok().body(room.user_id.to_string()))
    })
}

#[post("/list_room")]
pub async fn list_room(
    session: Session,
    pool: web::Data<DbPoolType>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    handle(&session,  req, |id| {
        let room_operator = RoomsOperator { conn: &pool };
        let result = room_operator.list_room(id.parse::<i32>().unwrap()).unwrap();
        Ok(HttpResponse::Ok().json(result))
    })
}
#[post("/delete_room")]
pub async fn delete_room(
    session: Session,
    pool: web::Data<DbPoolType>,
    room: web::Json<PostRoom>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    handle(&session, req, |id| {
        let room_operator = RoomsOperator { conn: &pool };
        room_operator
            .delete_room(id.parse::<i32>().unwrap(), room.room_id.parse::<i32>().unwrap())
            .unwrap();
        Ok(HttpResponse::Ok().body(id))
    })
}

pub fn handle<F>(
    session: &Session,
    req: HttpRequest,
    handle: F,
) -> Result<HttpResponse, Error>
where
    F: Fn(String) -> Result<HttpResponse, Error>,
{
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap(), &session)
            {
                Some(id) => handle(id),
                None => Ok(HttpResponse::Forbidden().body("token not exist")),
            }
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}
