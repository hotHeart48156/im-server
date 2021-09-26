
// use diesel:
use crate::schema::users;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub gender: Option<i16>,
    pub password: String,
    pub avater:Option<String>
}

#[derive(Insertable, AsExpression)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub gender: i16,
    pub password: &'a str,
}

#[derive(Insertable, AsExpression)]
#[table_name = "users"]
pub struct Avater<'a> {
    pub avater: &'a str,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostUser {
    pub name: String,
    // pub gender:Option<i16> ,
    pub password: String,
}
#[derive(Debug,Default,Clone,Serialize, Deserialize)]
pub struct TokenUser {
    pub exp: usize,
    pub id: i32,
    pub name: String,
    pub gender: Option<i16>,
}
impl TokenUser {
    pub fn new() -> TokenUser {
        TokenUser {
            exp:86400,
            ..Default::default()
        }
    }
}
