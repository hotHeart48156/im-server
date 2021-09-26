use actix_web::{post, web, HttpResponse};

use crate::{db::user_operator::UserOperator, models::user_model::PostUser, server::DbPoolType};

#[post("/register")]
pub async fn register(
    pool: web::Data<DbPoolType>,
    userinfo: web::Json<PostUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_operator = UserOperator { conn: &pool };
    let user = user_operator
        .get_user_by_name_and_password(userinfo.name.to_owned(), userinfo.password.to_owned());
    if let None=user {
        match user_operator.new_user(userinfo.name.as_str(), userinfo.password.as_str(), 0) {
            Ok(_) => {},
            Err(_) => {},
        };
        Ok(HttpResponse::Ok().body("ok"))
    }else {
        Ok(HttpResponse::Conflict().body("user exist"))
    }
    
}
