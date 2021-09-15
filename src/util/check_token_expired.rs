use actix_session::Session;

use crate::models::user_model::{TokenUser};

use super::token::Token;
pub fn check_user_token_is_expired(token: &str) -> Option<String> {
   match Token::default()
           .decode_token::<TokenUser>(token.to_string()) {
       Ok(tk) => {Some(tk.claims.id.to_string()) },
       Err(_) => {None},
   }
}
