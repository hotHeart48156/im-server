use crate::{
    db::friend_operator::FriendOperator,
    models::friend_model::{NewFriend, PostFriend},
    server::DbPoolType,
    util::{check_token_expired::check_user_token_is_expired, get_token::get_token},
};
use actix_session::Session;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
#[post("/add_friend")]
pub async fn add_friend(
    session: Session,
    pool: web::Data<DbPoolType>,
    friend: web::Json<PostFriend>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    handle(
        &session,
        pool.to_owned(),
        friend.to_owned(),
        req,
        move |id| {
            let new_friend = NewFriend {
                user_id: id.parse::<i32>().unwrap(),
                friend_id: friend.friend_id.parse::<i32>().unwrap(),
            };
            let friend_operator = FriendOperator { conn: &pool };
            friend_operator.add_friends(new_friend).unwrap();
            Ok(HttpResponse::Ok().body(id))
        },
    )
}

#[post("/list_friend")]
pub async fn list_friend(
    session: Session,
    pool: web::Data<DbPoolType>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    handle(
        &session,
        pool.to_owned(),
        PostFriend::default(),
        req,
        |id| {
            let friend_operator = FriendOperator { conn: &pool };
            let result = friend_operator
                .list_friends(id.parse::<i32>().unwrap())
                .unwrap();
            Ok(HttpResponse::Ok().json(result))
        },
    )
}

#[post("/delete_friend")]
pub async fn delete_friend(
    session: Session,
    pool: web::Data<DbPoolType>,
    friend: web::Json<PostFriend>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    handle(
        &session,
        pool.to_owned(),
        PostFriend::default(),
        req,
        |id| {
            let friend_operator = FriendOperator { conn: &pool };
            let result = friend_operator.delete_friend(
                id.parse::<i32>().unwrap(),
                friend.friend_id.parse::<i32>().unwrap(),
            );
            match result {
                Some(ok) => Ok(HttpResponse::Ok().body(ok.to_string())),
                None => Ok(HttpResponse::Gone().body("not found friend id")),
            }
        },
    )
}

pub fn handle<F>(
    session: &Session,
    pool: web::Data<DbPoolType>,
    friend: PostFriend,
    req: HttpRequest,
    handle: F,
) -> Result<HttpResponse, Error>
where
    F: Fn(String) -> Result<HttpResponse, Error>,
{
    let token = get_token(&req);
    match token {
        Some(tk) => {
            match check_user_token_is_expired(std::str::from_utf8(tk.as_bytes()).unwrap())
            {
                Some(id) => {
                    let new_friend = NewFriend {
                        user_id: id.parse::<i32>().unwrap(),
                        friend_id: friend.friend_id.parse::<i32>().unwrap(),
                    };
                    let friend_operator = FriendOperator { conn: &pool };
                    friend_operator.add_friends(new_friend).unwrap();
                    handle(id)
                }
                None => Ok(HttpResponse::Forbidden().body("token is expired")),
            }
        }
        None => Ok(HttpResponse::Forbidden().body("token is expired")),
    }
}
