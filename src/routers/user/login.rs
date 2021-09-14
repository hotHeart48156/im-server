use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};


use crate::{db::user_operator::UserOperator, models::user_model::PostUser, server::DbPoolType};
#[post("/login")]
pub async fn login(
    pool: web::Data<DbPoolType>,
    userinfo: web::Json<PostUser>,
    session:Session
) -> Result<HttpResponse, Error> {
    let user_operator = UserOperator { conn: &pool };
    let user = user_operator
        .get_user_by_name_and_password(userinfo.name.to_owned(), userinfo.password.to_owned());
    if let Some(user) = user {
        let token=gen_token(user.clone()).await.unwrap();
        println!("{}",userinfo.name.as_str());
        session.set(user.id.to_owned().to_string().as_str(), token.clone())?;
        return Ok(HttpResponse::Ok().body(token));
    }else {
        Ok(HttpResponse::NotFound().body("cant find user"))
    }
    
}

 async fn gen_token<'a, T>(ser: T) -> Result<String,Box<dyn std::error::Error>>
where
    T: Serialize + Deserialize<'a>,
{   let key=EncodingKey::from_secret("secret".as_ref());
    let mut header = Header::new(Algorithm::HS512);
    header.kid=Some("csdcsdcs".to_string());
    let token=encode(&header, &ser,& key);

    Ok(token.unwrap())
}
