use crate::{
    db::friend_operator::FriendOperator, models::friend_model::PostFriend, server::DbPoolType,
};
use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
#[post("/add_friend")]
pub async fn add_friend(
    session: Session,
    pool: web::Data<DbPoolType>,
    friend: web::Json<PostFriend>,
) -> Result<HttpResponse, Error> {
    match session
        .get::<String>(friend.user_id.to_string().as_str())
        .unwrap()
    {
        Some(_) => {
            let friend_operator = FriendOperator { conn: &pool };
            friend_operator
                .add_friends(friend.user_id, friend.friend_id)
                .unwrap();
            Ok(HttpResponse::Ok().body(friend.user_id.to_string()))
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    userid: String,
}

#[post("/list_friend")]
pub async fn list_friend(
    session: Session,
    pool: web::Data<DbPoolType>,
    friend: web::Json<Id>,
) -> Result<HttpResponse, Error> {
    match session
        .get::<String>(friend.userid.to_string().as_str())
        .unwrap()
    {
        Some(_) => {
            let friend_operator = FriendOperator { conn: &pool };
            let result=friend_operator
                .list_friends(friend.userid.parse::<i32>().unwrap())
                .unwrap();
            let body=serde_json::to_string(&result).unwrap();
            Ok(HttpResponse::Ok().body(body))
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}

#[post("/delete_friend")]
pub async fn delete_friend(
    session: Session,
    pool: web::Data<DbPoolType>,
    friend: web::Json<PostFriend>,
) -> Result<HttpResponse, Error> {
    match session
        .get::<String>(friend.user_id.to_string().as_str())
        .unwrap()
    {
        Some(_) => {
            let friend_operator = FriendOperator { conn: &pool };
            let result = friend_operator.delete_friend(friend.user_id, friend.friend_id);
            match result {
                Some(ok) => Ok(HttpResponse::Ok().body(ok.to_string())),
                None => Ok(HttpResponse::Gone().body("not found friend id")),
            }
        }
        None => Ok(HttpResponse::Forbidden().body("token not exist")),
    }
}
