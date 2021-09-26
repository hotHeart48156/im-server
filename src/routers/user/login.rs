use crate::{
    db::user_operator::UserOperator,
    models::user_model::{PostUser, TokenUser},
    server::DbPoolType,
    util::token::Token,
};
use actix_web::{post, web, Error, HttpResponse};
use chrono::Local;
#[post("/login")]
pub async fn login(
    pool: web::Data<DbPoolType>,
    userinfo: web::Json<PostUser>,
) -> Result<HttpResponse, Error> {
    let user_operator = UserOperator { conn: &pool };
    let user = user_operator
        .get_user_by_name_and_password(userinfo.name.to_owned(), userinfo.password.to_owned());
    
        match user {
        Some(recv_user) => {
            let week=std::env::var("USER_TOKEN_EXP_WEEK").unwrap().parse::<i64>().unwrap();
            let duration=chrono::Duration::weeks(week);
            let exp=Local::now().checked_add_signed(duration).expect("msg");
            let token = Token::default()
                .gen_token::<TokenUser>(&TokenUser {
                    exp: exp.timestamp() as usize,
                    id:recv_user.id,
                    name:recv_user.name,
                    gender:recv_user.gender,
                })
                .unwrap();
            // session.set(recv_user.id.to_owned().to_string().as_str(), token.clone())?;
            Ok(HttpResponse::Ok().body(token))
        }
        None => Ok(HttpResponse::NotAcceptable().body("cant find user")),
    }
}
