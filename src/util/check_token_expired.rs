use actix_session::Session;

use crate::models::user_model::{TokenUser};

use super::token::Token;
pub fn check_user_token_is_expired(token: &str, session: &Session) -> Option<String> {
    let tk = Token::default()
        .decode_token::<TokenUser>(token.to_string())
        .unwrap();


        
    let redis_token = session
        .get::<String>(tk.claims.id.to_string().as_str())
        .unwrap();
    match redis_token {
        Some(_) => Some(tk.claims.id.to_string()),
        None => None,
    }
}
