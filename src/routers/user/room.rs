use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};

use crate::{db::room_operator::RoomsOperator, models::rooms_model::{PostRoom, Room}, server::DbPoolType};
#[post("/add_room")]
pub async fn add_room(
    session: Session,
    pool: web::Data<DbPoolType>,
    room: web::Json<PostRoom>,
) -> Result<HttpResponse, Error> {
    match session
        .get::<String>(room.user_id.to_string().as_str())
        .unwrap()
    {
        Some(_) => {
            let room_operator = RoomsOperator { conn: &pool };
            room_operator
                .create_room(room.user_id)
                .unwrap();
            Ok(HttpResponse::Ok().body(room.user_id.to_string()))
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}

#[post("/list_room")]
pub async fn list_room(
    session: Session,
    pool: web::Data<DbPoolType>,
    room: web::Json<PostRoom>,
) -> Result<HttpResponse, Error> {
    match session
        .get::<String>(room.user_id.to_string().as_str())
        .unwrap()
    {
        Some(_) => {
            let room_operator = RoomsOperator { conn: &pool };
            let result=room_operator
                .list_room(room.user_id)
                .unwrap();
                let body=serde_json::to_string(&result).unwrap();
            Ok(HttpResponse::Ok().body(body))
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}
#[post("/delete_room")]
pub async fn delete_room(
    session: Session,
    pool: web::Data<DbPoolType>,
    room: web::Json<Room>,
) -> Result<HttpResponse, Error> {
    match session
        .get::<String>(room.user_id.to_string().as_str())
        .unwrap()
    {
        Some(_) => {
            let room_operator = RoomsOperator { conn: &pool };
            room_operator
                .delete_room(room.user_id,room.id)
                .unwrap();
            Ok(HttpResponse::Ok().body(room.user_id.to_string()))
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}
