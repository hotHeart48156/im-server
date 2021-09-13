use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};
use futures::TryFutureExt;
use hmac::{Hmac, NewMac};
use jwt::{Header, SignWithKey, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

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
        let token=gen_token(user).unwrap_or_else(|_|{"".to_string()}).await;
        session.set(userinfo.name.as_str(), token.clone())?;
        return Ok(HttpResponse::Ok().body(token));
    }else {
        Ok(HttpResponse::NotFound().body("cant find user"))
    }
    
}

pub async fn gen_token<'a, T>(ser: T) -> Result<String, Box<dyn std::error::Error>>
where
    T: Serialize + Deserialize<'a>,
{
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let header = Header {
        algorithm: jwt::AlgorithmType::Es384,
        ..Default::default()
    };
    let unsigned_token = Token::new(header, ser);
    let signed_token:String = unsigned_token
        .sign_with_key(&key)
        .map_err(|_e| "Sign error")?.into();

    Ok(signed_token)
}
